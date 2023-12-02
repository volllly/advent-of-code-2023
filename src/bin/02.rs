use chumsky::{
    prelude::*,
    text::{keyword, newline},
};
use tailsome::IntoOption;

advent_of_code::solution!(2);

#[derive(Debug)]
enum Colors {
    Red(u32),
    Green(u32),
    Blue(u32),
}

#[derive(Debug)]
struct Game {
    id: u32,
    draws: Vec<Draw>,
}

#[derive(Default, Debug, PartialEq, Eq)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

fn game_parser() -> impl Parser<char, Vec<Game>, Error = Simple<char>> {
    let id = keyword("Game")
        .padded()
        .ignore_then(text::int::<_, Simple<_>>(10).map(|s: String| s.parse::<u32>().unwrap()));

    let draws = text::int::<char, Simple<_>>(10)
        .map(|s: String| s.parse::<u32>().unwrap())
        .padded()
        .then(text::ident())
        .map(|(count, name)| match name.as_str() {
            "blue" => Colors::Blue(count),
            "green" => Colors::Green(count),
            "red" => Colors::Red(count),
            _ => panic!("Unknown color {}", name),
        })
        .separated_by(just(',').padded())
        .map(|draw| {
            draw.iter().fold(Draw::default(), |acc, draw| match *draw {
                Colors::Red(red) => Draw { red, ..acc },
                Colors::Green(green) => Draw { green, ..acc },
                Colors::Blue(blue) => Draw { blue, ..acc },
            })
        })
        .separated_by(just(';').padded());

    id.then_ignore(just(":").padded())
        .then(draws)
        .map(|(id, draws)| Game { id, draws })
        .separated_by(newline())
}

pub fn part_one(input: &str) -> Option<u32> {
    let games = game_parser().parse(input).unwrap();

    let bag = Draw {
        red: 12,
        green: 13,
        blue: 14,
    };

    games
        .iter()
        .filter_map(|Game { id, draws }| {
            if draws
                .iter()
                .any(|draw| draw.red > bag.red || draw.green > bag.green || draw.blue > bag.blue)
            {
                None
            } else {
                id.into_some()
            }
        })
        .sum::<u32>()
        .into_some()
}

pub fn part_two(input: &str) -> Option<u32> {
    let games = game_parser().parse(input).unwrap();

    games
        .iter()
        .map(|Game { id: _, draws }| {
            let max = draws.iter().fold(Draw::default(), |acc, draw| Draw {
                red: draw.red.max(acc.red),
                green: draw.green.max(acc.green),
                blue: draw.blue.max(acc.blue),
            });
            max.red * max.green * max.blue
        })
        .sum::<u32>()
        .into_some()
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
        assert_eq!(result, 2286.into_some());
    }
}
