use itertools::Itertools;
use std::collections::HashSet;

pub fn part1(input: &str) -> usize {
    let mut matrix = parse(input);

    let mut res = 0;
    for _ in 0..100 {
        res += step(&mut matrix);
    }

    res
}

pub fn part2(input: &str) -> usize {
    let mut matrix = parse(input);
    let size = matrix.iter().flatten().count();

    let mut i = 0;
    loop {
        i += 1;
        if step(&mut matrix) == size {
            return i;
        }
    }
}

// change matrix in place and return number of flashes
fn step(matrix: &mut [Vec<u8>]) -> usize {
    let width = matrix[0].len();
    let height = matrix.len();

    let mut flashing: Vec<(usize, usize)> = Vec::new();
    let mut flashed: HashSet<(usize, usize)> = HashSet::new();

    // charge up all squids
    for (x, y) in (0..width).cartesian_product(0..height) {
        matrix[y][x] += 1;
        if matrix[y][x] == 10 {
            flashing.push((x, y));
        }
    }

    // flash all squids
    while let Some((x, y)) = flashing.pop() {
        flashed.insert((x, y));

        for (dx, dy) in util::grid::get_adjacent_with_diag(x, y, width, height) {
            matrix[dy][dx] += 1;
            if matrix[dy][dx] == 10 {
                flashing.push((dx, dy));
            }
        }
    }

    // reset flashed squids
    for &(x, y) in flashed.iter() {
        matrix[y][x] = 0;
    }

    flashed.len()
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex() {
        let input = "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

        assert_eq!(part1(input), 1656);
    }

    #[test]
    fn part2_ex() {
        let input = "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

        assert_eq!(part2(input), 195);
    }
}
