use std::collections::VecDeque;

use itertools::Itertools;

pub fn part1(input: &str) -> usize {
    let conditional_records = parse(input);
    conditional_records
        .into_iter()
        .map(|(row, rules)| count_arrangements(row, rules))
        .sum()
}

pub fn part2(_input: &str) -> u64 {
    0
}

fn count_arrangements(row: &str, rules: Vec<usize>) -> usize {
    generate_variations(row)
        .iter()
        .filter(|variation| check_variation(variation, &rules))
        .count()
}

fn generate_variations(row: &str) -> Vec<String> {
    let mut queue: VecDeque<String> = VecDeque::from([row.to_owned()]);
    let mut variations = Vec::new();

    while let Some(row) = queue.pop_front() {
        if let Some((operational, broken)) = perform_replacement(&row) {
            queue.push_back(operational);
            queue.push_back(broken);
        } else {
            variations.push(row);
        }
    }

    variations
}

fn perform_replacement(row: &str) -> Option<(String, String)> {
    for (i, ch) in row.char_indices() {
        if ch == '?' {
            let mut replace_operational = row.chars().collect_vec();
            replace_operational[i] = '.';

            let mut replace_broken = row.chars().collect_vec();
            replace_broken[i] = '#';

            return Some((
                replace_operational.iter().collect(),
                replace_broken.iter().collect(),
            ));
        }
    }

    None
}

fn check_variation(variation: &str, rules: &[usize]) -> bool {
    let lengths = variation.split('.')
        .map(|group| group.len())
        .filter(|n| n != &0)
        .collect_vec();

    &lengths == rules
}

fn parse(input: &str) -> Vec<(&str, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let (row, rules) = line.split_once(' ').unwrap();
            let rules = rules.split(',').map(|n| n.parse().unwrap()).collect();
            (row, rules)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../examples/day12.txt");

    #[test]
    fn generate_variations_test() {
        assert_eq!(
            vec!["...", "..#", ".#.", ".##", "#..", "#.#", "##.", "###"],
            generate_variations("???")
        );
    }

    #[test]
    fn check_variation_test() {
        assert!(check_variation("#.#.###", &vec![1, 1, 3]));
        assert!(!check_variation("..#.###", &vec![1, 1, 3]));
    }

    #[test]
    fn count_arrangements_test() {
        assert_eq!(1, count_arrangements("???.###", vec![1, 1, 3]));
        assert_eq!(4, count_arrangements(".??..??...?##.", vec![1, 1, 3]));
        assert_eq!(1, count_arrangements("?#?#?#?#?#?#?#?", vec![1, 3, 1, 6]));
        assert_eq!(1, count_arrangements("????.#...#...", vec![4, 1, 1]));
        assert_eq!(4, count_arrangements("????.######..#####.", vec![1, 6, 5]));
        assert_eq!(10, count_arrangements("?###????????", vec![3, 2, 1]));
    }

    #[test]
    fn part1_ex() {
        assert_eq!(21, part1(EXAMPLE));
    }

    #[test]
    fn part2_ex() {}
}
