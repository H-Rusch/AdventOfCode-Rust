use std::collections::HashMap;

use itertools::Itertools;

pub fn part1(input: &str) -> String {
    let index_map = build_index_map(input);

    let message_length = input.lines().next().unwrap().len();
    (0..message_length)
        .map(|i| find_most_common(index_map.get(&i).unwrap()))
        .collect()
}

pub fn part2(input: &str) -> String {
    let index_map = build_index_map(input);

    let message_length = input.lines().next().unwrap().len();
    (0..message_length)
        .map(|i| find_least_common(index_map.get(&i).unwrap()))
        .collect()
}

fn build_index_map(input: &str) -> HashMap<usize, Vec<char>> {
    let mut index_map = HashMap::new();

    for line in input.lines() {
        for (i, c) in line.char_indices() {
            index_map.entry(i).or_insert(Vec::new()).push(c);
        }
    }

    index_map
}

fn find_most_common(chars: &[char]) -> char {
    let counts = chars.iter().counts();
    **counts.keys().max_by_key(|c| counts.get(**c)).unwrap()
}

fn find_least_common(chars: &[char]) -> char {
    let counts = chars.iter().counts();
    **counts.keys().min_by_key(|c| counts.get(**c)).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../examples/day06.txt");

    #[test]
    fn part1_ex() {
        assert_eq!("easter", part1(INPUT));
    }

    #[test]
    fn part2_ex() {
        assert_eq!("advent", part2(INPUT));
    }
}
