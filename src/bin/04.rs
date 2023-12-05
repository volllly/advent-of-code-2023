use std::{collections::HashSet, ops::Deref, str::FromStr};

use chumsky::{prelude::*, text::newline};
use tailsome::{IntoOption, IntoResult};

advent_of_code::solution!(4);

#[derive(Debug)]
struct Card {
    id: u32,
    winning: HashSet<u32>,
    numbers: HashSet<u32>,
}

impl Card {
    pub fn parser() -> impl Parser<char, Card, Error = Simple<char>> {
        let id = just("Card")
            .padded()
            .ignore_then(text::int(10).map(|s: String| s.parse::<u32>().unwrap()))
            .then_ignore(just(':').padded());

        let cards = text::int(10)
            .map(|s: String| s.parse::<u32>().unwrap())
            .separated_by(just(' ').repeated())
            .collect::<HashSet<_>>();

        id.then(cards.then_ignore(just('|').padded()))
            .then(cards)
            .map(|((id, winning), numbers)| Card {
                id,
                winning,
                numbers,
            })
    }
}

impl FromStr for Card {
    type Err = Vec<Simple<char>>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Card::parser().parse(s)
    }
}

#[derive(Debug)]
struct Cards(Vec<Card>);

impl Deref for Cards {
    type Target = Vec<Card>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Cards {
    type Err = Vec<Simple<char>>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self(Card::parser().separated_by(newline()).parse(s)?).into_ok()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let cards = Cards::from_str(input).unwrap();

    cards
        .iter()
        .map(
            |Card {
                 id: _,
                 winning,
                 numbers,
             }| {
                match numbers
                    .iter()
                    .filter(|number| winning.contains(number))
                    .count()
                {
                    0 => 0,
                    number => 2u32.pow(number as u32 - 1),
                }
            },
        )
        .sum::<u32>()
        .into_some()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use tailsome::IntoOption;

    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, 13.into_some());
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
