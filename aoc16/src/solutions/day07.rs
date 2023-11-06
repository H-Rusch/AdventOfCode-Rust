use std::collections::HashSet;

use itertools::Itertools;

#[derive(Debug)]
struct IPv7 {
    supernet_sequences: Vec<String>,
    hypernet_sequences: Vec<String>,
}

impl IPv7 {
    fn from(line: &str) -> Self {
        let mut basic_sequences = Vec::new();
        let mut hypernet_sequences = Vec::new();

        let mut supernet = true;
        let mut chars = line.chars().peekable();
        while chars.peek().is_some() {
            let sequence = IPv7::build_sequence(&mut chars);
            match supernet {
                true => basic_sequences.push(sequence),
                false => hypernet_sequences.push(sequence),
            }
            supernet = !supernet;
        }

        IPv7 {
            supernet_sequences: basic_sequences,
            hypernet_sequences,
        }
    }

    fn build_sequence(chars: &mut impl Iterator<Item = char>) -> String {
        let mut sequence = String::new();

        for ch in chars.by_ref() {
            if ch == '[' || ch == ']' {
                break;
            }

            sequence.push(ch);
        }

        sequence
    }
}

pub fn part1(input: &str) -> usize {
    parse(input)
        .iter()
        .filter(|address| supports_tls(address))
        .count()
}

pub fn part2(input: &str) -> usize {
    parse(input)
        .iter()
        .filter(|address| supports_ssl(address))
        .count()
}

fn supports_tls(address: &IPv7) -> bool {
    if address
        .hypernet_sequences
        .iter()
        .any(|sequence| supports_abba(sequence))
    {
        return false;
    }

    address
        .supernet_sequences
        .iter()
        .any(|sequence| supports_abba(sequence))
}

fn supports_abba(sequence: &str) -> bool {
    sequence
        .chars()
        .tuple_windows()
        .any(|(a, b, c, d)| a == d && b == c && a != b)
}

fn supports_ssl(address: &IPv7) -> bool {
    let bab_pairings: HashSet<(char, char, char)> = address
        .hypernet_sequences
        .iter()
        .flat_map(|sequence| aba_pairings(sequence))
        .collect();
    address
        .supernet_sequences
        .iter()
        .flat_map(|sequence| aba_pairings(sequence))
        .any(|(a, b, _)| bab_pairings.contains(&(b, a, b)))
}

fn aba_pairings(sequence: &str) -> impl Iterator<Item = (char, char, char)> + '_ {
    sequence
        .chars()
        .tuple_windows()
        .filter(|&(a, b, c)| a == c && a != b)
}

fn parse(input: &str) -> Vec<IPv7> {
    input.lines().map(IPv7::from).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_TSL: &str = include_str!("../../examples/day07_tsl.txt");
    const EXAMPLE_SSL: &str = include_str!("../../examples/day07_ssl.txt");

    #[test]
    fn support_tls_test() {
        assert!(supports_tls(&IPv7::from("abba[mnop]qrst")));
        assert!(!supports_tls(&IPv7::from("abcd[bddb]xyyx")));
        assert!(!supports_tls(&IPv7::from("aaaa[qwer]tyui")));
        assert!(supports_tls(&IPv7::from("ioxxoj[asdfgh]zxcvbn")));
    }

    #[test]
    fn part1_ex() {
        assert_eq!(2, part1(EXAMPLE_TSL));
    }

    #[test]
    fn support_ssl_test() {
        assert!(supports_ssl(&IPv7::from("aba[bab]xyz")));
        assert!(!supports_ssl(&IPv7::from("xyx[xyx]xyx")));
        assert!(supports_ssl(&IPv7::from("aaa[kek]eke")));
        assert!(supports_ssl(&IPv7::from("zazbz[bzb]cdb")));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(3, part2(EXAMPLE_SSL));
    }
}
