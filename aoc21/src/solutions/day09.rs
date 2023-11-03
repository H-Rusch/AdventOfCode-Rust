use itertools::Itertools;
use std::collections::HashSet;

pub fn part1(input: &str) -> u32 {
    let grid = parse(input); 

    find_low_points(&grid).iter()
        .map(|&(x, y)| grid[y][x] + 1)
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let grid = parse(input); 

    find_low_points(&grid).iter()
        .map(|&(x, y)| calculate_basin_area(x, y, &grid))
        .sorted()
        .rev()
        .take(3)
        .product()
}

fn calculate_basin_area(x: usize, y: usize, grid: &Vec<Vec<u32>>) -> u32 {
    let height = grid.len();
    let width = grid[0].len();

    let mut area = 0;
    let mut stack = vec![(x, y)];
    let mut seen: HashSet<(usize, usize)> = HashSet::from([(x, y)]);

    while let Some((x, y)) = stack.pop() {
        if grid[y][x] == 9 {
            continue;
        }
        area += 1;

        for (dx, dy) in util::grid::get_adjacent(x, y, width, height) {
            if !seen.contains(&(dx, dy)) {
                seen.insert((dx, dy));
                stack.push((dx, dy));
            }
        }
    }

    area
}

fn find_low_points(grid: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let height = grid.len();
    let width = grid[0].len();

    (0..width).cartesian_product(0..height)
        .filter(|&(x, y)| util::grid::get_adjacent(x, y, width, height).all(|(dx, dy)| grid[dy][dx] > grid[y][x]))
        .collect()
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars()
            .map(|c| c.to_digit(10).unwrap() as u32)
            .collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex() {
        let input = "\
2199943210
3987894921
9856789892
8767896789
9899965678";

        assert_eq!(15, part1(input));
    } 

    #[test]
    fn part2_ex() {
        let input = "\
2199943210
3987894921
9856789892
8767896789
9899965678";

        assert_eq!(1134, part2(input));
    }


    #[test]
    fn find_correct_basin_size() {
        let input = "\
2199943210
3987894921
9856789892
8767896789
9899965678";
        let grid = parse(input);

        assert_eq!(calculate_basin_area(2, 2, &grid), 14);

    }
}
