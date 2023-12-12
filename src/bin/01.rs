use std::collections::BTreeMap;

use tailsome::IntoOption;

advent_of_code::solution!(1);

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
