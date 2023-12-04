use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
struct Card {
    id: usize,
    winning: HashSet<usize>,
    numbers: HashSet<usize>,
}

impl Card {
    fn matching_count(&self) -> usize {
        self.winning.intersection(&self.numbers).count()
    }

    fn calc_points(&self) -> usize {
        let winning_count = self.matching_count();

        if winning_count == 0 {
            return 0;
        }
        2_usize.pow(winning_count as u32 - 1)
    }
}

pub fn part1(input: &str) -> usize {
    let cards = parse(input);
    cards.iter().map(Card::calc_points).sum()
}

pub fn part2(input: &str) -> usize {
    let cards = parse(input);
    let mut card_counts: HashMap<usize, usize> = cards.iter().map(|card| (card.id, 1)).collect();

    for card in cards {
        let count = *card_counts.get(&card.id).unwrap();
        for i in 1..=card.matching_count() {
            *card_counts.entry(card.id + i).or_insert(1) += count;
        }
    }

    card_counts.values().sum()
}

fn parse(input: &str) -> Vec<Card> {
    let card_pattern = Regex::new(r"Card\s*(\d+):\s*([\d\s]+)\s*\|\s*([\d\s]+)").unwrap();
    input
        .lines()
        .map(|line| {
            let captures = card_pattern.captures(line).unwrap();
            let id = captures.get(1).unwrap().as_str().parse().unwrap();
            let winning = captures
                .get(2)
                .unwrap()
                .as_str()
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            let numbers = captures
                .get(3)
                .unwrap()
                .as_str()
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();

            Card {
                id,
                winning,
                numbers,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../examples/day04.txt");

    #[test]
    fn scratchcard_point_calculation_test() {
        let card = Card {
            id: 0,
            winning: HashSet::from([41, 48, 83, 86, 17]),
            numbers: HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]),
        };

        assert_eq!(8, card.calc_points());
    }

    #[test]
    fn part1_ex() {
        assert_eq!(13, part1(EXAMPLE));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(30, part2(EXAMPLE));
    }
}
