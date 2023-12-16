use itertools::Itertools;
use util::grid::Coordinate;

pub fn part1(input: &str) -> u64 {
    sum_galaxy_distances(input, 2)
}

pub fn part2(input: &str) -> u64 {
    sum_galaxy_distances(input, 1_000_000)
}

fn sum_galaxy_distances(input: &str, distance_multiplier: i32) -> u64 {
    let galaxies = parse(input, distance_multiplier);

    galaxies
        .iter()
        .tuple_combinations()
        .map(|(c1, c2)| c1.manhatten_distance(c2) as u64)
        .sum()
}

fn parse(input: &str, expansionn_factor: i32) -> Vec<Coordinate> {
    let mut galaxies = Vec::new();
    let mut dy = 0;

    for (y, row) in input.lines().enumerate() {
        if !row_contains_galaxy(row) {
            dy += 1;
            continue;
        }
        let mut dx = 0;
        for (x, cell) in row.chars().enumerate() {
            if !col_contains_galaxy(input, x) {
                dx += 1;
                continue;
            }

            if cell == '#' {
                galaxies.push(Coordinate::from(
                    x as i32 + dx * (expansionn_factor - 1),
                    y as i32 + dy * (expansionn_factor - 1),
                ))
            }
        }
    }

    galaxies
}

fn row_contains_galaxy(row: &str) -> bool {
    row.chars().any(|ch| ch == '#')
}

fn col_contains_galaxy(input: &str, x: usize) -> bool {
    input
        .lines()
        .any(|line| line.chars().collect::<Vec<_>>()[x] == '#')
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../examples/day11.txt");

    #[test]
    fn part1_ex() {
        assert_eq!(374, part1(EXAMPLE));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(1030, sum_galaxy_distances(EXAMPLE, 10));
        assert_eq!(8410, sum_galaxy_distances(EXAMPLE, 100));
    }
}
