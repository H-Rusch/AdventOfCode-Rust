use fancy_regex::{Captures, Regex};
use lazy_static::lazy_static;
use std::error::Error;

trait FindDigit {
    fn find_first_digit(&self, line: &str) -> Option<char>;
    fn find_last_digit(&self, line: &str) -> Option<char>;
}
struct NumbersOnly;

impl FindDigit for NumbersOnly {
    fn find_first_digit(&self, line: &str) -> Option<char> {
        line.chars().find(|ch| ch.is_ascii_digit())
    }

    fn find_last_digit(&self, line: &str) -> Option<char> {
        line.chars().rev().find(|ch| ch.is_ascii_digit())
    }
}

struct NumbersAndSpelled;

lazy_static! {
    static ref DIGIT_OR_SPELLED: Regex =
        Regex::new(r"(?=(\d|one|two|three|four|five|six|seven|eight|nine))").unwrap();
}

impl FindDigit for NumbersAndSpelled {
    fn find_first_digit(&self, line: &str) -> Option<char> {
        let mut matches = DIGIT_OR_SPELLED.captures_iter(line);

        let Some(captures) = matches.next() else {
            return None;
        };
        Some(get_captured_char(&captures.unwrap()))
    }

    fn find_last_digit(&self, line: &str) -> Option<char> {
        let matches = DIGIT_OR_SPELLED.captures_iter(line);

        let Some(captures) = matches.last() else {
            return None;
        };
        Some(get_captured_char(&captures.unwrap()))
    }
}

fn get_captured_char(captures: &Captures) -> char {
    match captures.get(1).unwrap().as_str() {
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9',
        digit => digit.chars().next().unwrap(),
    }
}

pub fn part1(input: &str) -> u32 {
    sum_calibration_numbers(input, &NumbersOnly)
}

pub fn part2(input: &str) -> u32 {
    sum_calibration_numbers(input, &NumbersAndSpelled)
}

fn sum_calibration_numbers(input: &str, finder: &impl FindDigit) -> u32 {
    input
        .lines()
        .map(|line| find_calibration_number(line, finder).unwrap())
        .sum()
}

fn find_calibration_number(line: &str, finder: &impl FindDigit) -> Result<u32, Box<dyn Error>> {
    let front = finder.find_first_digit(line).ok_or("No digit found")?;
    let back = finder.find_last_digit(line).ok_or("No digit found")?;

    let calibration_number = String::from_iter([front, back]);

    Ok(calibration_number.parse::<u32>()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("../../examples/day01_1.txt");
    const EXAMPLE2: &str = include_str!("../../examples/day01_2.txt");

    #[test]
    fn part1_ex() {
        assert_eq!(142, part1(EXAMPLE1));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(281, part2(EXAMPLE2));
    }

    #[test]
    fn overlapping_spelled_out_numbers() {
        let finder = NumbersAndSpelled;
        let result = find_calibration_number("eighthree", &finder);

        assert_eq!(83, result.unwrap());
    }
}
