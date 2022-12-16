use std::collections::HashSet;

pub fn part1(input: &str) -> usize {
    input.lines()
        .map(|line| {
            let mid_point = line.len() / 2;
            let (left, right) = line.split_at(mid_point);

            let left: HashSet<char> = HashSet::from_iter(left.chars());
            let right: HashSet<char> = HashSet::from_iter(right.chars());

            let item = left.intersection(&right).next().unwrap();
            get_priority(item)
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    input.lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunk| {
            let items = chunk.iter()
                .map(|items| HashSet::from_iter(items.chars()))
                .reduce(|left, right| left.intersection(&right).copied().collect::<HashSet<char>>())
                .unwrap();
            let item = items.iter().next().unwrap();
            get_priority(item)
        })
        .sum()
}

fn get_priority(item: &char) -> usize {
    match item {
        'a'..='z' => *item as usize + 1 - 97,
        'A'..='Z' => *item as usize + 27 - 65,
        _ => unreachable!(),
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../examples/day03.txt");

    #[test]
    fn part1_ex() {
        assert_eq!(157, part1(INPUT));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(70, part2(INPUT));
    }
}
