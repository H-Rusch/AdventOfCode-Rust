use std::ops::RangeInclusive;

use rayon::prelude::*;

pub fn part1(input: &str) -> u32 {
    let ranges = parse(input);
    find_lowest_matching(&ranges).unwrap()
}

pub fn part2(input: &str) -> usize {
    let ranges = parse(input);
    count_allowed(&ranges)
}

fn find_lowest_matching(ranges: &[RangeInclusive<u32>]) -> Option<u32> {
    (u32::MIN..=u32::MAX).find(|i| adress_allowed(ranges, i))
}

fn adress_allowed(ranges: &[RangeInclusive<u32>], adress: &u32) -> bool {
    !ranges.iter().any(|range| range.contains(adress))
}

fn count_allowed(ranges: &[RangeInclusive<u32>]) -> usize {
    (u32::MIN..=u32::MAX)
        .into_par_iter()
        .filter(|i| adress_allowed(ranges, i))
        .count()
}

fn parse(input: &str) -> Vec<RangeInclusive<u32>> {
    input
        .lines()
        .map(|line| {
            let (low, high) = line.split_once('-').unwrap();
            let (low, high) = (low.parse().unwrap(), high.parse().unwrap());

            low..=high
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../examples/day20.txt");

    #[test]
    fn adress_allowed_test() {
        let ranges = parse(INPUT);

        let allowed = [3, 9];
        let disallowed = [0, 1, 2, 4, 5, 6, 7, 8];
        for i in allowed {
            assert!(adress_allowed(&ranges, &i));
        }
        for i in disallowed {
            assert!(!adress_allowed(&ranges, &i));
        }
    }
}
