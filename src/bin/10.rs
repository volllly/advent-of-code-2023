use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Index},
    str::FromStr,
};

use chumsky::{prelude::*, text::newline};
use strum::EnumIs;
use tailsome::{IntoOption, IntoResult};
use tap::Tap;

advent_of_code::solution!(10);

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIs)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
struct Pipe([Direction; 2]);

impl TryFrom<char> for Pipe {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Pipe([Direction::North, Direction::East]),
            'F' => Pipe([Direction::South, Direction::East]),
            'J' => Pipe([Direction::North, Direction::West]),
            '7' => Pipe([Direction::South, Direction::West]),
            '|' => Pipe([Direction::North, Direction::South]),
            '-' => Pipe([Direction::East, Direction::West]),
            _ => ().into_err()?,
        }
        .into_ok()
    }
}

impl Pipe {
    fn is_north(&self) -> bool {
        self.0.iter().any(|d| d.is_north())
    }
    fn is_east(&self) -> bool {
        self.0.iter().any(|d| d.is_east())
    }
    fn is_south(&self) -> bool {
        self.0.iter().any(|d| d.is_south())
    }
    fn is_west(&self) -> bool {
        self.0.iter().any(|d| d.is_west())
    }

    fn directions(&self) -> &[Direction; 2] {
        &self.0
    }
}

impl Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_north() && self.is_east() {
            f.write_str("L")
        } else if self.is_east() && self.is_south() {
            f.write_str("F")
        } else if self.is_south() && self.is_west() {
            f.write_str("7")
        } else if self.is_west() && self.is_north() {
            f.write_str("J")
        } else if self.is_north() && self.is_south() {
            f.write_str("|")
        } else if self.is_west() && self.is_east() {
            f.write_str("-")
        } else {
            unreachable!();
        }
    }
}

#[derive(Debug, EnumIs)]
enum Cell {
    Empty,
    Start,
    Pipe(Pipe),
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => f.write_str("."),
            Cell::Start => f.write_str("S"),
            Cell::Pipe(pipe) => f.write_str(&pipe.to_string()),
        }
    }
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        if value == 'S' {
            Cell::Start
        } else {
            Pipe::try_from(value).map(Cell::Pipe).unwrap_or(Cell::Empty)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Coordinates {
    pub x: isize,
    pub y: isize,
}

impl Add<Direction> for Coordinates {
    type Output = Coordinates;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::North => Coordinates {
                y: self.y - 1,
                x: self.x,
            },
            Direction::East => Coordinates {
                y: self.y,
                x: self.x + 1,
            },
            Direction::South => Coordinates {
                y: self.y + 1,
                x: self.x,
            },
            Direction::West => Coordinates {
                y: self.y,
                x: self.x - 1,
            },
        }
    }
}

impl AddAssign<Direction> for Coordinates {
    fn add_assign(&mut self, rhs: Direction) {
        *self = *self + rhs;
    }
}

struct Cells(Vec<Vec<Cell>>);

impl Debug for Cells {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("\n")?;
        for row in self.rows() {
            for cell in row.iter() {
                f.write_str(&cell.to_string())?
            }
            f.write_str("\n")?;
        }

        Ok(())
    }
}

impl From<Vec<Vec<Cell>>> for Cells {
    fn from(value: Vec<Vec<Cell>>) -> Self {
        Self(value)
    }
}

impl Index<Coordinates> for Cells {
    type Output = Cell;

    fn index(&self, index: Coordinates) -> &Self::Output {
        if index.x < 0 || index.y < 0 {
            return &Cell::Empty;
        }

        self.0
            .get(index.y as usize)
            .and_then(|row| row.get(index.x as usize))
            .unwrap_or(&Cell::Empty)
    }
}

impl Cells {
    fn rows(&self) -> impl Iterator<Item = &Vec<Cell>> {
        self.0.iter()
    }

    fn onto(&self, from: Coordinates, through: Coordinates) -> Option<Coordinates> {
        let Cell::Pipe(cell) = &self[through] else {
            return None;
        };
        cell.directions()
            .map(|d| through + d)
            .iter()
            .find(|c| **c != from)
            .copied()
    }
}

#[derive(Debug)]
struct Network {
    cells: Cells,
    start: (Coordinates, Pipe),
}

impl From<Cells> for Network {
    fn from(cells: Cells) -> Self {
        let start = cells
            .rows()
            .enumerate()
            .find_map(|(y, r)| {
                if let Some(cell) = r.iter().enumerate().find_map(|(x, c)| {
                    if c.is_start() {
                        (x, c).into_some()
                    } else {
                        None
                    }
                }) {
                    (y, cell).into_some()
                } else {
                    None
                }
            })
            .map(|(y, (x, _))| Coordinates {
                x: x as isize,
                y: y as isize,
            })
            .unwrap();

        let mut directions = Vec::<Direction>::new();
        if let Cell::Pipe(pipe) = &cells[start + Direction::North] {
            if pipe.is_south() {
                directions.push(Direction::North)
            }
        }
        if let Cell::Pipe(pipe) = &cells[start + Direction::East] {
            if pipe.is_west() {
                directions.push(Direction::East)
            }
        }
        if let Cell::Pipe(pipe) = &cells[start + Direction::South] {
            if pipe.is_north() {
                directions.push(Direction::South)
            }
        }
        if let Cell::Pipe(pipe) = &cells[start + Direction::West] {
            if pipe.is_east() {
                directions.push(Direction::West)
            }
        }

        Network {
            start: (start, Pipe(directions.try_into().unwrap())),
            cells,
        }
    }
}

impl FromStr for Network {
    type Err = Vec<Simple<char>>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let row = newline().not().map(|c: char| Cell::from(c)).repeated();

        row.separated_by(newline())
            .map(Cells::from)
            .map(Network::from)
            .parse(s)
    }
}

impl Network {
    fn get_loop(&self) -> Vec<Coordinates> {
        let mut coordinates = vec![self.start.0];

        let mut last = self.start.0;
        let mut current = last + self.start.1.directions()[0];

        loop {
            coordinates.push(current);
            (last, current) = (current, self.cells.onto(last, current).unwrap());

            if current == self.start.0 {
                return coordinates;
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    (Network::from_str(input).unwrap().get_loop().len() / 2).into_some()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, 8.into_some());
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
