use core::panic;
use std::{collections::HashMap, str::FromStr};

use chumsky::{prelude::*, text::newline};
use slotmap::{new_key_type, SlotMap};
use tailsome::{IntoOption, IntoResult};

advent_of_code::solution!(8);

new_key_type! { struct Id; }

#[derive(Debug)]
struct Node {
    left: Id,
    right: Id,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct Map {
    directions: Vec<Direction>,
    map: SlotMap<Id, Node>,
}

impl FromStr for Map {
    type Err = Vec<Simple<char>>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let directions = filter_map(|_, c: char| Direction::from(c).into_ok()).repeated();

        let node = text::ident().then_ignore(just('=').padded()).then(
            text::ident()
                .separated_by(just(','))
                .delimited_by(just('('), just(')')),
        );

        directions
            .then_ignore(newline().repeated())
            .then(node.separated_by(newline()))
            .map(|(directions, nodes)| {
                let mut ids = HashMap::<String, Id>::new();
                let mut map = SlotMap::<Id, Option<Node>>::with_key();
                for node in nodes {
                    let left = node.1[0];
                    let left = if let Some(id) = ids.get(&left) {
                        *id
                    } else {
                        let id = map.insert(None);
                        ids.insert(left, id);
                        id
                    };
                    let right = node.1[1];
                    let right = if let Some(id) = ids.get(&right) {
                        *id
                    } else {
                        let id = map.insert(None);
                        ids.insert(right, id);
                        id
                    };

                    map[if let Some(id) = ids.get(&node.0) {
                        *id
                    } else {
                        let id = map.insert(None);
                        ids.insert(node.0, id);
                        id
                    }] = Node { left, right }.into_some();
                }

                Map { directions, map: map.into_iter().map(|(i, n)| (i, n.unwrap())).collect()}
                            })
            .parse(s)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
