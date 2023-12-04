use std::collections::BTreeMap;

use tailsome::IntoOption;

advent_of_code::solution!(1);

// fn token(parse_written: bool) -> Box<dyn Parser<char, u32, Error = Simple<char>>> {
//     let num = filter(|c: &char| c.is_ascii_digit()).map(|c| c.to_digit(10).unwrap());

//     let digit = |(text, number)| just(text).map(move |_| number);

//     let text = digit(("one", 1))
//         .or(digit(("two", 2)))
//         .or(digit(("three", 3)))
//         .or(digit(("four", 4)))
//         .or(digit(("five", 5)))
//         .or(digit(("six", 6)))
//         .or(digit(("seven", 7)))
//         .or(digit(("eight", 8)))
//         .or(digit(("nine", 9)));

//     if parse_written {
//         Box::new(text.or(num))
//     } else {
//         Box::new(num)
//     }
// }

// fn line(parse_written: bool) -> impl Parser<char, Vec<u32>, Error = Simple<char>> {
//     take_until(token(parse_written)).map(|t| t.1).repeated()
// }

// fn callibration(input: &str, parse_written: bool) -> Result<u32, Vec<Simple<char>>> {
//     let parser = line(parse_written);

//     input
//         .lines()
//         .inspect(|line| println!("Parsing: {}", line))
//         .map(|line| parser.parse(line))
//         .inspect(|line| println!("Parsed : {:?}", line))
//         .collect::<Result<Vec<_>, _>>()?
//         .into_iter()
//         .filter(|line| !line.is_empty())
//         .fold(0, |acc, line| {
//             acc + 10 * line.first().unwrap() + line.last().unwrap()
//         })
//         .into_ok()
// }

fn parse_line(input: &str, parse_written: bool) -> Option<(u32, u32)> {
    let digits = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let mut parsed = BTreeMap::<usize, u32>::new();

    for digit in digits {
        for (index, _) in input.match_indices(&digit.1.to_string()) {
            parsed.insert(index, digit.1);
        }

        if parse_written {
            for (index, _) in input.match_indices(&digit.0) {
                parsed.insert(index, digit.1);
            }
        }
    }

    if parsed.is_empty() {
        None
    } else {
        (
            *parsed.first_key_value().unwrap().1,
            *parsed.last_key_value().unwrap().1,
        )
            .into_some()
    }
}

fn callibration(input: &str, parse_written: bool) -> u32 {
    input
        .lines()
        .filter_map(|line| parse_line(line, parse_written))
        .fold(0, |acc, line| acc + 10 * line.0 + line.1)
}

pub fn part_one(input: &str) -> Option<u32> {
    callibration(input, false).into_some()
}

pub fn part_two(input: &str) -> Option<u32> {
    callibration(input, true).into_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
