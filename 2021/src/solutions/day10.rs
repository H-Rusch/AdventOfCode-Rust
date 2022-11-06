use std::collections::HashMap;

use itertools::Itertools;

enum BraketResult {
    Error(char),
    NotClosed(Vec<char>),
}

pub fn part1(input: &str) -> u32 {
    let values: HashMap<char, u32> = HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137),
    ]);
    input.lines()
        .map(find_error)
        .filter_map(|opt| match opt {
            BraketResult::Error(c) => Some(c),
            BraketResult::NotClosed(_) => None,
        })
        .map(|c| values.get(&c).unwrap())
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let sorted_scores: Vec<u64> = input.lines()
        .filter_map(|line| match find_error(line) {
            BraketResult::NotClosed(vec) => Some(vec),
            BraketResult::Error(_) => None,
        })
        .map(calc_closing)
        .sorted()
        .collect();

    sorted_scores[sorted_scores.len() / 2]
}

fn calc_closing(stack: Vec<char>) -> u64 {
    let values: HashMap<char, u64> = HashMap::from([
        ('(', 1),
        ('[', 2),
        ('{', 3),
        ('<', 4),
    ]);
    stack.iter()
        .rev()
        .map(|c| *values.get(c).unwrap())
        .reduce(|v1, v2| v1 * 5 + v2)
        .unwrap_or_default()
}

fn find_error(line: &str) -> BraketResult {
    let pairs: HashMap<char, char> = HashMap::from([
        (']', '['),
        ('>', '<'),
        (')', '('),
        ('}', '{'),
    ]);
    let mut stack: Vec<char> = Vec::new();
    for c in line.chars() {
        if pairs.contains_key(&c) {
            if stack.last().unwrap() == pairs.get(&c).unwrap() {
                stack.pop();
            } else {
                return BraketResult::Error(c);
            }
        } else {
            stack.push(c);
        }
    }

    BraketResult::NotClosed(stack)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex() {
        let input = "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

        assert_eq!(part1(input), 26397);
    }

    #[test]
    fn part2_ex() {
        let input = "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

        assert_eq!(part2(input), 288957);
    }

    #[test]
    fn calc_closing_correctly() {
        let stack = vec!['[', '(', '{', '(', '[', '[', '{', '{'];

        assert_eq!(calc_closing(stack), 288957);
    }
}
