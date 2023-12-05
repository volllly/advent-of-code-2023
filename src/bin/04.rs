use std::{
    collections::{BTreeMap, HashMap, HashSet},
    ops::{Add, Deref},
    str::FromStr,
};

use chumsky::{prelude::*, text::newline};
use tailsome::{IntoOption, IntoResult};

advent_of_code::solution!(4);

#[derive(Debug)]
struct Card {
    winning: HashSet<u32>,
    numbers: HashSet<u32>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
struct Id(usize);

impl Add<usize> for Id {
    type Output = Id;

    fn add(self, rhs: usize) -> Self::Output {
        Id(self.0 + rhs)
    }
}

impl Card {
    pub fn parser() -> impl Parser<char, (Id, Card), Error = Simple<char>> {
        let id = just("Card")
            .padded()
            .ignore_then(text::int(10).map(|s: String| Id(s.parse::<usize>().unwrap())))
            .then_ignore(just(':').padded());

        let cards = text::int(10)
            .map(|s: String| s.parse::<u32>().unwrap())
            .separated_by(just(' ').repeated())
            .collect::<HashSet<_>>();

        id.then(cards.then_ignore(just('|').padded()))
            .then(cards)
            .map(|((id, winning), numbers)| (id, Card { winning, numbers }))
    }

    pub fn winning(&self) -> usize {
        self.numbers
            .iter()
            .filter(|number| self.winning.contains(number))
            .count()
    }
}

#[derive(Debug)]
struct Cards(BTreeMap<Id, Card>);

impl Deref for Cards {
    type Target = BTreeMap<Id, Card>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Cards {
    type Err = Vec<Simple<char>>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self(
            Card::parser()
                .separated_by(newline())
                .parse(s)?
                .into_iter()
                .collect::<BTreeMap<Id, Card>>(),
        )
        .into_ok()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let cards = Cards::from_str(input).unwrap();

    cards
        .iter()
        .map(|(_, card)| match card.winning() {
            0 => 0,
            number => 2u32.pow(number as u32 - 1),
        })
        .sum::<u32>()
        .into_some()
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards = Cards::from_str(input).unwrap();
    let mut counts = cards
        .iter()
        .map(|(id, _)| (*id, 1))
        .collect::<HashMap<Id, u32>>();

    for (id, card) in (*cards).iter() {
        let count = *counts.get(id).unwrap();
        for n in 1..(card.winning() + 1) {
            *counts.get_mut(&(*id + n)).unwrap() += count;
        }
    }

    counts.values().sum::<u32>().into_some()
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
        assert_eq!(result, 30.into_some());
    }
}
