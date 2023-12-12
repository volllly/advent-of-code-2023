use std::{ops::Deref, str::FromStr};

use chumsky::{prelude::*, text::newline};
use tailsome::IntoOption;

advent_of_code::solution!(6);

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

trait AssertPart {
    const VALID: ();
}

struct Races<const P: usize>(Vec<Race>);

impl<const P: usize> AssertPart for Races<P> {
    const VALID: () = assert!(P >= 1 && P <= 2);
}

impl<const P: usize> Deref for Races<P> {
    type Target = Vec<Race>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const P: usize> From<Vec<Race>> for Races<P> {
    fn from(value: Vec<Race>) -> Self {
        Self(value)
    }
}

impl<const P: usize> Races<P> {
    fn numbers_parser() -> Box<dyn Parser<char, Vec<u64>, Error = Simple<char>>> {
        match P {
            1 => Box::new(
                text::int(10)
                    .map(|s: String| s.parse::<u64>().unwrap())
                    .separated_by(just(' ').repeated()),
            ),
            2 => Box::new(text::int(10).separated_by(just(' ').repeated()).map(
                |s: Vec<String>| {
                    vec![s
                        .into_iter()
                        .reduce(|a, b| a + &b)
                        .unwrap()
                        .parse::<u64>()
                        .unwrap()]
                },
            )),
            _ => panic!(),
        }
    }
}

impl<const P: usize> FromStr for Races<P> {
    type Err = Vec<Simple<char>>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        just("Time:")
            .padded()
            .ignore_then(Self::numbers_parser())
            .then_ignore(newline())
            .then_ignore(just("Distance:").padded())
            .then(Self::numbers_parser())
            .map(|(times, distances)| {
                times
                    .into_iter()
                    .zip(distances)
                    .map(|(time, distance)| Race { time, distance })
                    .collect::<Vec<_>>()
                    .into()
            })
            .parse(s)
    }
}

impl Race {
    fn permutations(&self) -> impl Iterator<Item = u64> + '_ {
        (1..=self.time)
            .map(|time| {
                let remaining = self.time - time;
                time * remaining
            })
            .filter(|distance| *distance > self.distance)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    Races::<1>::from_str(input)
        .unwrap()
        .iter()
        .map(|r| r.permutations().count() as u64)
        .product::<u64>()
        .into_some()
}

pub fn part_two(input: &str) -> Option<u64> {
    Races::<2>::from_str(input)
        .unwrap()
        .iter()
        .map(|r| r.permutations().count() as u64)
        .product::<u64>()
        .into_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, 288.into_some());
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, 71503.into_some());
    }
}
