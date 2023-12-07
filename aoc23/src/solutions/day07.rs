use std::cmp::Ordering;

use itertools::Itertools;

#[derive(Eq)]
struct Hand {
    cards: Cards,
    hand_type: HandType,
    bet: usize,
}

impl Hand {
    fn new(cards: String, bet: usize) -> Self {
        let hand_type = HandType::from(&cards);
        let cards = Cards::new(&cards, Box::new(SimpleEvaluator));
        Self {
            cards,
            hand_type,
            bet,
        }
    }

    fn new_with_wildcard(cards: String, bet: usize) -> Self {
        let hand_type = HandType::get_highest_for_wildcard(&cards);
        let cards = Cards::new(&cards, Box::new(WildcardEvaluator));
        Self {
            cards,
            hand_type,
            bet,
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self.hand_type == other.hand_type
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> Ordering {
        self.hand_type
            .cmp(&other.hand_type)
            .then(self.cards.cmp(&other.cards))
    }
}

#[derive()]
struct Cards {
    cards: Vec<char>,
    evaluator: Box<dyn EvaluateCardValue>,
}

impl Cards {
    fn new(cards: &str, evaluator: Box<dyn EvaluateCardValue>) -> Self {
        let cards = cards.chars().collect();
        Self { cards, evaluator }
    }
}

impl PartialEq for Cards {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Cards {}

impl PartialOrd for Cards {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Cards {
    fn cmp(&self, other: &Cards) -> Ordering {
        self.cards
            .iter()
            .zip(other.cards.iter())
            .map(|(card1, card2)| {
                (
                    self.evaluator.card_value(*card1),
                    self.evaluator.card_value(*card2),
                )
            })
            .map(|(card_val1, card_val2)| usize::cmp(&card_val1, &card_val2))
            .find(|ord| *ord != Ordering::Equal)
            .unwrap_or(Ordering::Equal)
    }
}

trait EvaluateCardValue {
    fn card_value(&self, card: char) -> usize;
}

struct SimpleEvaluator;

impl EvaluateCardValue for SimpleEvaluator {
    fn card_value(&self, card: char) -> usize {
        "23456789TJQKA".chars().position(|ch| ch == card).unwrap()
    }
}

struct WildcardEvaluator;

impl EvaluateCardValue for WildcardEvaluator {
    fn card_value(&self, card: char) -> usize {
        "J23456789TQKA".chars().position(|ch| ch == card).unwrap()
    }
}

#[derive(Eq, PartialEq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn from(cards: &str) -> Self {
        let counts = cards.chars().counts();

        if counts.values().contains(&5) {
            return HandType::FiveOfAKind;
        }
        if counts.values().contains(&4) {
            return HandType::FourOfAKind;
        }
        if counts.values().contains(&3) && counts.values().contains(&2) {
            return HandType::FullHouse;
        }
        if counts.values().contains(&3) {
            return HandType::ThreeOfAKind;
        }
        if counts.values().filter(|v| v == &&2).count() == 2 {
            return HandType::TwoPair;
        }
        if counts.values().contains(&2) {
            return HandType::OnePair;
        }
        HandType::HighCard
    }

    fn get_highest_for_wildcard(cards: &str) -> Self {
        "AKQT98765432"
            .chars()
            .map(|ch| cards.replace('J', &ch.to_string()))
            .map(|replaced| HandType::from(&replaced))
            .max()
            .unwrap()
    }

    fn value(&self) -> usize {
        match *self {
            HandType::FiveOfAKind => 7,
            HandType::FourOfAKind => 6,
            HandType::FullHouse => 5,
            HandType::ThreeOfAKind => 4,
            HandType::TwoPair => 3,
            HandType::OnePair => 2,
            HandType::HighCard => 1,
        }
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &HandType) -> Ordering {
        self.value().cmp(&other.value())
    }
}

pub fn part1(input: &str) -> usize {
    let mut hands = parse(input);
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bet)
        .sum()
}

pub fn part2(input: &str) -> usize {
    let mut hands = parse_wildcard(input);
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bet)
        .sum()
}

fn parse(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|line| {
            let (hand, bet) = line.split_once(' ').unwrap();
            Hand::new(hand.to_string(), bet.parse().unwrap())
        })
        .collect()
}

fn parse_wildcard(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|line| {
            let (hand, bet) = line.split_once(' ').unwrap();
            Hand::new_with_wildcard(hand.to_string(), bet.parse().unwrap())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../examples/day07.txt");

    #[test]
    fn hand_ordering_different_types_test() {
        let hand1 = Hand::new("32T3K".to_string(), 765);
        let hand2 = Hand::new("T55J5".to_string(), 685);

        assert!(hand2 > hand1);
    }

    #[test]
    fn hand_ordering_same_type_test() {
        let hand1 = Hand::new("KK677".to_string(), 28);
        let hand2 = Hand::new("KTJJT".to_string(), 220);

        assert!(hand2 < hand1);
    }

    #[test]
    fn wildcard_parsing_test() {
        assert_eq!(
            HandType::OnePair,
            Hand::new_with_wildcard("32T3k".to_string(), 0).hand_type
        );
        assert_eq!(
            HandType::TwoPair,
            Hand::new_with_wildcard("KK6//".to_string(), 0).hand_type
        );
        assert_eq!(
            HandType::FourOfAKind,
            Hand::new_with_wildcard("T55J5".to_string(), 0).hand_type
        );
        assert_eq!(
            HandType::FourOfAKind,
            Hand::new_with_wildcard("KTJJT".to_string(), 0).hand_type
        );
        assert_eq!(
            HandType::FourOfAKind,
            Hand::new_with_wildcard("QQQJA".to_string(), 0).hand_type
        );
    }

    #[test]
    fn part1_ex() {
        assert_eq!(6440, part1(EXAMPLE));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(5905, part2(EXAMPLE));
    }
}
