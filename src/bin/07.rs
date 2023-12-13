use std::{cmp::Ordering, collections::HashMap, str::FromStr};

use chumsky::{prelude::*, text::newline};
use tailsome::{IntoOption, IntoResult};

advent_of_code::solution!(7);

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Card<const P: u8> {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl<const P: u8> Card<P> {
    fn rank(self) -> u8 {
        *match P {
            1 => [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            2 => [1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 10, 11, 12],
            _ => panic!(),
        }
        .get(self as usize)
        .unwrap()
    }
}

impl<const P: u8> PartialOrd for Card<P> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.rank().cmp(&other.rank()))
    }
}

impl<const P: u8> Ord for Card<P> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank().cmp(&other.rank())
    }
}

impl<const P: u8> TryFrom<char> for Card<P> {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Jack,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => return Err(()),
        }
        .into_ok()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Value {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand<const P: u8> {
    cards: [Card<P>; 5],
    value: Value,
    bid: u64,
}

impl<const P: u8> From<([Card<P>; 5], u64)> for Hand<P> {
    fn from(from: ([Card<P>; 5], u64)) -> Self {
        let set = from
            .0
            .iter()
            .map(|v| (*v, from.0.into_iter().filter(|x| *x == *v).count()))
            .collect::<HashMap<Card<P>, usize>>();

        let mut value = if set.iter().any(|(_, count)| *count == 5) {
            Value::FiveOfAKind
        } else if set.iter().any(|(_, count)| *count == 4) {
            Value::FourOfAKind
        } else if set.iter().any(|(_, count)| *count == 3)
            && set.iter().any(|(_, count)| *count == 2)
        {
            Value::FullHouse
        } else if set.iter().any(|(_, count)| *count == 3) {
            Value::ThreeOfAKind
        } else if set.iter().filter(|(_, count)| **count == 2).count() == 2 {
            Value::TwoPair
        } else if set.iter().any(|(_, count)| *count == 2) {
            Value::OnePair
        } else {
            Value::HighCard
        };

        if P == 2 {
            if let Some(jokers) = set.get(&Card::Jack).copied() {
                value = match value {
                    Value::FiveOfAKind => Value::FiveOfAKind,
                    Value::FourOfAKind => Value::FiveOfAKind,
                    Value::FullHouse => Value::FiveOfAKind,
                    Value::ThreeOfAKind => Value::FourOfAKind,
                    Value::TwoPair => match jokers {
                        2 => Value::FourOfAKind,
                        1 => Value::FullHouse,
                        _ => panic!(),
                    },
                    Value::OnePair => Value::ThreeOfAKind,
                    Value::HighCard => Value::OnePair,
                }
            }
        }

        Self {
            cards: from.0,
            value,
            bid: from.1,
        }
    }
}

impl<const P: u8> PartialOrd for Hand<P> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const P: u8> Ord for Hand<P> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self.value as usize).cmp(&(other.value as usize)) {
            Ordering::Equal => self
                .cards
                .iter()
                .zip(other.cards)
                .map(|(a, b)| a.cmp(&b))
                .find(|o| o.is_ne())
                .unwrap(),
            order => order,
        }
    }
}

#[derive(Debug)]
struct Game<const P: u8>(Vec<Hand<P>>);

impl<const P: u8> From<Vec<([Card<P>; 5], u64)>> for Game<P> {
    fn from(value: Vec<([Card<P>; 5], u64)>) -> Self {
        Self(value.into_iter().map(Hand::from).collect::<Vec<_>>())
    }
}

impl<const P: u8> AsMut<Vec<Hand<P>>> for Game<P> {
    fn as_mut(&mut self) -> &mut Vec<Hand<P>> {
        &mut self.0
    }
}

impl<const P: u8> FromStr for Game<P> {
    type Err = Vec<Simple<char>>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        filter_map(|_, c: char| Card::try_from(c).unwrap().into_ok())
            .repeated()
            .exactly(5)
            .then_ignore(just(' '))
            .then(text::int(10).map(|s: String| s.parse::<u64>().unwrap()))
            .map(|(cards, bid)| (cards.as_slice().try_into().unwrap(), bid))
            .separated_by(newline())
            .map(Game::from)
            .parse(s)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut game = Game::<1>::from_str(input).unwrap();
    game.as_mut().sort();
    game.as_mut()
        .iter()
        .enumerate()
        .map(|(index, hand)| hand.bid * (index as u64 + 1))
        .sum::<u64>()
        .into_some()
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut game = Game::<2>::from_str(input).unwrap();
    game.as_mut().sort();
    game.as_mut()
        .iter()
        .enumerate()
        .map(|(index, hand)| hand.bid * (index as u64 + 1))
        .sum::<u64>()
        .into_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, 6440.into_some());
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, 5905.into_some());
    }
}
