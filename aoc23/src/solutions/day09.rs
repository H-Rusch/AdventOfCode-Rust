use itertools::Itertools;

pub fn part1(input: &str) -> i32 {
    let histories = parse(input);
    histories.iter().map(|history| predict_next_end(history)).sum()
}

pub fn part2(input: &str) -> i32 {
    let histories = parse(input);
    histories.iter().map(|history| predict_next_start(history)).sum()
}

fn predict_next_end(history: &[i32]) -> i32 {
    if history.iter().all(|diff| diff == &0) {
        return 0;
    }

    let differences: Vec<i32> = history.iter().tuple_windows().map(|(x, y)| y - x).collect();
    let predicted_diff = predict_next_end(&differences);

    *history.last().unwrap() + predicted_diff
}

fn predict_next_start(history: &[i32]) -> i32 {
    if history.iter().all(|diff| diff == &0) {
        return 0;
    }

    let differences: Vec<i32> = history.iter().tuple_windows().map(|(x, y)| y - x).collect();
    let predicted_diff = predict_next_start(&differences);

    *history.first().unwrap() - predicted_diff
}

fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../examples/day09.txt");

    #[test]
    fn predict_next_end_test() {
        assert_eq!(18, predict_next_end(&vec![0, 3, 6, 9, 12, 15]));
        assert_eq!(28, predict_next_end(&vec![1, 3, 6, 10, 15, 21]));
        assert_eq!(68, predict_next_end(&vec![10, 13, 16, 21, 30, 45]));
        assert_eq!(-2, predict_next_end(&vec![0, 1, 1, 0]));
    }

     #[test]
    fn predict_next_start_test() {
        assert_eq!(-3, predict_next_start(&vec![0, 3, 6, 9, 12, 15]));
        assert_eq!(0, predict_next_start(&vec![1, 3, 6, 10, 15, 21]));
        assert_eq!(5, predict_next_start(&vec![10, 13, 16, 21, 30, 45]));
    }

    #[test]
    fn part1_ex() {
        assert_eq!(114, part1(EXAMPLE));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(2, part2(EXAMPLE));
    }
}
