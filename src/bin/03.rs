use std::{
    collections::HashSet,
    fmt::{Debug, Write},
    iter::repeat,
    ops::Range,
    str::FromStr,
};

use chumsky::{prelude::*, text::newline};
use slotmap::{new_key_type, SlotMap};
use tailsome::IntoOption;

advent_of_code::solution!(3);

struct Grid {
    lines: Vec<Line>,
    numbers: SlotMap<Number, u32>,
}

impl Grid {
    pub fn number(&self, number: &Number) -> u32 {
        self.numbers[*number]
    }

    pub fn lines(&self) -> impl Iterator<Item = &Line> {
        self.lines.iter()
    }

    pub fn cell(&self, line: usize, cell: usize) -> Option<Cell> {
        self.lines
            .get(line)
            .and_then(|line| line.cells.get(cell))
            .copied()
    }

    pub fn gear_ratio(&self, line_index: usize, cell_index: usize) -> Option<u32> {
        let mut numbers = HashSet::<Number>::new();
        if let Some(Cell::Symbol('*')) = self.cell(line_index, cell_index) {
            for y in (0..3).map(|r| r - 1) {
                for x in (0..3).map(|r| r - 1) {
                    if line_index as i32 >= -y && cell_index as i32 >= -x {
                        if let Some(Cell::Number(number)) = self.cell(
                            (line_index as i32 + y) as usize,
                            (cell_index as i32 + x) as usize,
                        ) {
                            numbers.insert(number);
                        }
                    }
                }
            }

            if numbers.len() == 2 {
                return numbers
                    .iter()
                    .map(|number| self.number(number))
                    .product::<u32>()
                    .into_some();
            }
        }

        None
    }

    pub fn part_number(&self, line_index: usize, cell_index: usize) -> Option<Number> {
        if let Some(Cell::Number(number)) = self.cell(line_index, cell_index) {
            for y in (0..3).map(|r| r - 1) {
                for x in (0..3).map(|r| r - 1) {
                    if line_index as i32 >= -y
                        && cell_index as i32 >= -x
                        && self
                            .cell(
                                (line_index as i32 + y) as usize,
                                (cell_index as i32 + x) as usize,
                            )
                            .map(Cell::is_symbol)
                            .unwrap_or(false)
                    {
                        return number.into_some();
                    }
                }
            }
        }

        None
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut printed = HashSet::<Number>::new();
        for (y, line) in self.lines().enumerate() {
            for (x, cell) in line.cells.iter().enumerate() {
                match cell {
                    Cell::Empty => f.write_str(". ")?,
                    Cell::Symbol(symbol) => f.write_str(&format!(
                        "{}{}",
                        symbol,
                        if self.gear_ratio(y, x).is_some() {
                            '|'
                        } else {
                            ' '
                        }
                    ))?,
                    Cell::Number(number) => {
                        if !printed.contains(number) {
                            for (n, char) in self.number(number).to_string().chars().enumerate() {
                                f.write_char(char)?;
                                f.write_char(if self.part_number(y, x + n).is_some() {
                                    '_'
                                } else {
                                    ' '
                                })?;
                            }
                            printed.insert(*number);
                        }
                    }
                }
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

impl From<Vec<Vec<ParserCell>>> for Grid {
    fn from(value: Vec<Vec<ParserCell>>) -> Self {
        let mut lines = Vec::new();
        let mut numbers = SlotMap::<Number, u32>::with_key();

        for cells in value {
            let mut line = Line::default();

            for cell in cells {
                match cell {
                    ParserCell::Empty => line.cells.push(Cell::Empty),
                    ParserCell::Symbol(symbol) => line.cells.push(Cell::Symbol(symbol)),
                    ParserCell::Number(((start, end), value)) => {
                        let key = numbers.insert(value);
                        line.cells
                            .extend(repeat(Cell::Number(key)).take(end - start));
                    }
                }
            }

            lines.push(line);
        }

        Self { lines, numbers }
    }
}

impl FromStr for Grid {
    type Err = Vec<Simple<char>>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        just('.')
            .to(ParserCell::Empty)
            .or(
                text::int::<char, Simple<_>>(10).map_with_span(|c: String, span: Range<usize>| {
                    let value = c.parse::<u32>().unwrap();

                    ParserCell::Number(((span.start, span.end), value))
                }),
            )
            .or(newline().not().map(|c: char| ParserCell::Symbol(c)))
            .repeated()
            .separated_by(newline())
            .map(Grid::from)
            .parse(s)
    }
}

new_key_type! { struct Number; }

#[derive(Debug, Default)]
struct Line {
    pub cells: Vec<Cell>,
}

#[derive(Clone, Debug)]
enum ParserCell {
    Empty,
    Symbol(char),
    Number(((usize, usize), u32)),
}

#[derive(Clone, Debug, Copy)]
enum Cell {
    Empty,
    Symbol(char),
    Number(Number),
}

impl Cell {
    pub fn is_symbol(self) -> bool {
        matches!(self, Cell::Symbol(_))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::from_str(input).unwrap();
    let mut part_numbers = HashSet::<Number>::new();

    for (y, line) in grid.lines().enumerate() {
        for (x, _) in line.cells.iter().enumerate() {
            if let Some(number) = grid.part_number(y, x) {
                part_numbers.insert(number);
            }
        }
    }

    part_numbers
        .iter()
        .map(|number| grid.number(number))
        .sum::<u32>()
        .into_some()
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::from_str(input).unwrap();
    let mut gear_ratios = 0;

    for (y, line) in grid.lines().enumerate() {
        for (x, _) in line.cells.iter().enumerate() {
            if let Some(ratio) = grid.gear_ratio(y, x) {
                gear_ratios += ratio;
            }
        }
    }

    gear_ratios.into_some()
}

#[cfg(test)]
mod tests {
    use tailsome::IntoOption;

    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, 4361.into_some());
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, 467835.into_some());
    }
}
