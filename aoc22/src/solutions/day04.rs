use num_traits::PrimInt;
use regex::Regex;
use std::ops::RangeInclusive;

trait OverlappingRange<T: PrimInt> {
    // check if calling range contains the given range
    fn contains_range(&self, other: &RangeInclusive<T>) -> bool;

    // check if calling range partially overlaps the given range
    fn partial_overlap(&self, other: &RangeInclusive<T>) -> bool;
}

impl<T: PrimInt> OverlappingRange<T> for RangeInclusive<T> {
    fn contains_range(&self, other: &RangeInclusive<T>) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }

    fn partial_overlap(&self, other: &RangeInclusive<T>) -> bool {
        self.contains(other.start()) || self.contains(other.end())
    }
}

pub fn part1(input: &str) -> usize {
    let ranges = parse(input);

    ranges
        .iter()
        .filter(|(range1, range2)| range1.contains_range(range2) || range2.contains_range(range1))
        .count()
}

pub fn part2(input: &str) -> usize {
    let ranges = parse(input);

    ranges
        .iter()
        .filter(|(range1, range2)| range1.partial_overlap(range2) || range2.partial_overlap(range1))
        .count()
}

pub fn parse(input: &str) -> Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> {
    let number_regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();

    input
        .lines()
        .map(|line| {
            let capture = number_regex.captures_iter(line).next().unwrap();
            let numbers: Vec<u32> = (1..5).map(|i| capture[i].parse().unwrap()).collect();

            let range1 = numbers[0]..=numbers[1];
            let range2 = numbers[2]..=numbers[3];

            (range1, range2)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part1_ex() {
        assert_eq!(2, part1(INPUT));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(4, part2(INPUT));
    }
}
