pub fn part1(input: &str) -> usize {
    let loads = parse(input);

    *loads.iter().max().unwrap()
}

pub fn part2(input: &str) -> usize {
    let mut loads = parse(input);
    loads.sort();

    loads.iter().rev().take(3).sum()
}

fn parse(input: &str) -> Vec<usize> {
    input
        .split("\n\n")
        .map(|batch| {
            batch
                .lines()
                .map(|line| line.parse::<usize>().unwrap())
                .sum()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../examples/day01.txt");

    #[test]
    fn part1_ex() {
        assert_eq!(24000, part1(INPUT));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(45000, part2(INPUT));
    }
}
