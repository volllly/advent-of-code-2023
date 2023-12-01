use chumsky::prelude::*;
use tailsome::IntoResult;

advent_of_code::solution!(1);

fn token(parse_written: bool) -> Box<dyn Parser<char, u32, Error = Simple<char>>> {
    let num = filter(|c: &char| c.is_ascii_digit()).map(|c| c.to_digit(10).unwrap());

    let digit = |(text, number)| just(text).map(move |_| number);

    let text = digit(("one", 1))
        .or(digit(("two", 2)))
        .or(digit(("three", 3)))
        .or(digit(("four", 4)))
        .or(digit(("five", 5)))
        .or(digit(("six", 6)))
        .or(digit(("seven", 7)))
        .or(digit(("eight", 8)))
        .or(digit(("nine", 9)));

    if parse_written {
        Box::new(text.or(num))
    } else {
        Box::new(num)
    }
}

fn line(parse_written: bool) -> impl Parser<char, Vec<u32>, Error = Simple<char>> {
    take_until(token(parse_written)).map(|t| t.1).repeated()
}

fn callibration(input: &str, parse_written: bool) -> Result<u32, Vec<Simple<char>>> {
    let parser = line(parse_written);

    input
        .lines()
        .inspect(|line| println!("Parsing: {}", line))
        .map(|line| parser.parse(line))
        .inspect(|line| println!("Parsed : {:?}", line))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .filter(|line| !line.is_empty())
        .fold(0, |acc, line| {
            acc + 10 * line.first().unwrap() + line.last().unwrap()
        })
        .into_ok()
}

pub fn part_one(input: &str) -> Option<u32> {
    callibration(input, false).unwrap().into()
}

pub fn part_two(input: &str) -> Option<u32> {
    callibration(input, true).unwrap().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(
            advent_of_code::template::read_file("examples", DAY)
                .split("---")
                .next()
                .unwrap(),
        );
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(
            advent_of_code::template::read_file("examples", DAY)
                .split("---")
                .last()
                .unwrap(),
        );
        assert_eq!(result, Some(281));
    }
}
