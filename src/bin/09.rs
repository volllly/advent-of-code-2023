use std::{iter::repeat, ops::Deref, str::FromStr};

use chumsky::{prelude::*, text::newline};
use itertools::{FoldWhile, Itertools};
use tailsome::IntoOption;

advent_of_code::solution!(9);

#[derive(Debug)]
struct History(Vec<i32>);

impl From<Vec<i32>> for History {
    fn from(value: Vec<i32>) -> Self {
        Self(value)
    }
}

impl Deref for History {
    type Target = Vec<i32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl History {
    fn differentiate(&self) -> History {
        self.iter()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .collect::<Vec<_>>()
            .into()
    }

    fn is_zero(&self) -> bool {
        self.iter().all(|v| *v == 0)
    }
}

#[derive(Debug)]
struct Derivatives(Vec<History>);

impl From<Vec<History>> for Derivatives {
    fn from(value: Vec<History>) -> Self {
        Self(value)
    }
}

impl From<History> for Derivatives {
    fn from(value: History) -> Self {
        repeat(())
            .fold_while(vec![value], |mut acc, _| {
                let last = acc.last().unwrap().differentiate();

                if last.is_zero() {
                    FoldWhile::Done(acc)
                } else {
                    acc.push(last);
                    FoldWhile::Continue(acc)
                }
            })
            .into_inner()
            .into()
    }
}

impl Derivatives {
    fn predict(&self) -> i32 {
        self.0
            .iter()
            .rev()
            .fold(0, |acc, d| d.last().unwrap() + acc)
    }
}

#[derive(Debug)]
struct Histories(Vec<Derivatives>);

impl Histories {
    fn iter(&self) -> impl Iterator<Item = &Derivatives> {
        self.0.iter()
    }
}

impl FromStr for Histories {
    type Err = Vec<Simple<char>>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let history = just('-')
            .or_not()
            .then(text::int(10))
            .map(|(sign, s)| s.parse::<i32>().unwrap() * if sign.is_some() { -1 } else { 1 })
            .separated_by(just(' '))
            .at_least(1)
            .map(History)
            .map(Derivatives::from);

        history.separated_by(newline()).map(Histories).parse(s)
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    Histories::from_str(input)
        .unwrap()
        .iter()
        .map(|h| h.predict())
        .sum::<i32>()
        .into_some()
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
        assert_eq!(result, 114.into_some());
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
