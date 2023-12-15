use std::{collections::HashMap, str::FromStr};

use chumsky::{prelude::*, text::newline};
use num::integer::lcm;
use tailsome::{IntoOption, IntoResult};

advent_of_code::solution!(8);

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

impl Node {
    pub fn take(&self, direction: Direction) -> &str {
        match direction {
            Direction::Left => self.left.as_str(),
            Direction::Right => self.right.as_str(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => Err(())?,
        }
        .into_ok()
    }
}

#[derive(Debug)]
struct Map {
    directions: Vec<Direction>,
    map: HashMap<String, Node>,
}

impl FromStr for Map {
    type Err = Vec<Simple<char>>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let directions = filter_map(|span, c: char| {
            Direction::try_from(c)
                .map_err(|_| Simple::custom(span, format!("invalid direction {}", c)))
        })
        .repeated();

        let id = filter(|c: &char| c.is_ascii_alphanumeric())
            .repeated()
            .exactly(3)
            .map(|c| c.into_iter().collect::<String>());

        let node = id.then_ignore(just('=').padded()).then(
            id.separated_by(just(',').padded())
                .exactly(2)
                .delimited_by(just('('), just(')')),
        );

        directions
            .then_ignore(newline().repeated())
            .then(node.separated_by(newline()))
            .map(|(directions, nodes)| Map {
                directions,
                map: nodes
                    .into_iter()
                    .map(|(k, mut v)| {
                        (
                            k,
                            Node {
                                right: v.pop().unwrap(),
                                left: v.pop().unwrap(),
                            },
                        )
                    })
                    .collect(),
            })
            .parse(s)
    }
}

impl Map {
    fn follow<'a>(&'a self, period: usize, from: &'a str) -> Vec<&str> {
        let mut trail = Vec::<&str>::new();

        let mut current = from;
        for direction in self.directions.iter().cycle().skip(period) {
            current = self.map[current].take(*direction);
            trail.push(current);
            if current.ends_with('Z') {
                return trail;
            }
        }
        unreachable!()
    }

    fn period(&self, from: &str) -> usize {
        let start = self.follow(0, from);
        let repeat = self.follow(start.len(), start.last().unwrap());

        if start.len() != repeat.len() {
            panic!("the first loop always has the same length as the final loop");
        }
        repeat.len()
    }

    fn follow_ghost(&self) -> u64 {
        let starts = self
            .map
            .keys()
            .filter(|k| k.ends_with('A'))
            .collect::<Vec<_>>();

        let periods = starts
            .into_iter()
            .map(|s| self.period(s))
            .collect::<Vec<_>>();

        periods
            .iter()
            .map(|period| *period as u64)
            .reduce(lcm)
            .unwrap()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Map::from_str(input)
        .unwrap()
        .follow(0, "AAA")
        .len()
        .into_some()
}

pub fn part_two(input: &str) -> Option<u64> {
    Map::from_str(input).unwrap().follow_ghost().into_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, 6.into_some());
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, 6.into_some());
    }
}
