use itertools::Itertools;
use rayon::prelude::*;

pub fn part1(input: &str) -> usize {
    let (times, distances) = parse(input);

    (0..times.len())
        .map(|i| count_ways_to_win(times[i], distances[i]))
        .product()
}

pub fn part2(input: &str) -> usize {
    let (time, distance) = parse_kerning(input);
    count_ways_to_win(time, distance)
}

fn count_ways_to_win(total_time: usize, record: usize) -> usize {
    (0..total_time)
        .into_par_iter()
        .filter(|charge_time| beats_record(total_time, *charge_time, record))
        .count()
}

fn beats_record(total_time: usize, charge_time: usize, record: usize) -> bool {
    (total_time - charge_time) * charge_time > record
}

fn parse(input: &str) -> (Vec<usize>, Vec<usize>) {
    input
        .lines()
        .map(|line| {
            let (_, numbers) = line.split_once(':').unwrap();
            numbers
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect_tuple()
        .unwrap()
}

fn parse_kerning(input: &str) -> (usize, usize) {
    input
        .lines()
        .map(|line| {
            let (_, numbers) = line.split_once(':').unwrap();
            numbers.split_whitespace().join("").parse().unwrap()
        })
        .collect_tuple()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../examples/day06.txt");

    #[test]
    fn count_ways_to_win_test() {
        assert_eq!(4, count_ways_to_win(7, 9));
        assert_eq!(8, count_ways_to_win(15, 40));
        assert_eq!(9, count_ways_to_win(30, 200));
    }

    #[test]
    fn part1_ex() {
        assert_eq!(288, part1(EXAMPLE));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(71503, part2(EXAMPLE));
    }
}
