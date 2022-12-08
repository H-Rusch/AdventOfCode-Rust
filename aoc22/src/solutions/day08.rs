use itertools::Itertools;

pub fn part1(input: &str) -> usize {
    let grid = parse(input);
    let (width, height) = (grid[0].len(), grid.len());
    let mut visible = vec![vec![false; width]; height];

    for y in 0..height {
        mark_visible(&grid, &mut visible, 0, y, (1, 0));
        mark_visible(&grid, &mut visible, width - 1, y, (-1, 0));
    }

    for x in 0..width {
        mark_visible(&grid, &mut visible, x, 0, (0, 1));
        mark_visible(&grid, &mut visible, x, height - 1, (0, -1));
    }

    visible.iter().flatten().filter(|b| **b).count()
}

pub fn part2(input: &str) -> i32 {
    let grid = parse(input);
    let (width, height) = (grid[0].len(), grid.len());

    (0..width)
        .cartesian_product(0..height)
        .map(|(x, y)| scenic_score(&grid, x, y))
        .max()
        .unwrap()
}

fn scenic_score(grid: &[Vec<i32>], start_x: usize, start_y: usize) -> i32 {
    visible_tree_counts(grid, start_x, start_y).iter().product()
}

fn visible_tree_counts(grid: &[Vec<i32>], start_x: usize, start_y: usize) -> Vec<i32> {
    let tree_height = grid[start_y][start_x];
    let (width, height) = (grid[0].len() as i32, grid.len() as i32);
    let mut tree_counts = Vec::with_capacity(4);

    for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let mut count = 0;
        let (mut x, mut y) = (start_x, start_y);

        loop {
            let (next_x, next_y) = (x as i32 + dx, y as i32 + dy);
            if next_x < 0 || next_x >= width || next_y < 0 || next_y >= height {
                break;
            }
            (x, y) = (next_x as usize, next_y as usize);

            count += 1;

            if grid[y][x] >= tree_height {
                break;
            }
        }

        tree_counts.push(count);
    }

    tree_counts
}

fn mark_visible(
    grid: &[Vec<i32>],
    visible: &mut [Vec<bool>],
    start_x: usize,
    start_y: usize,
    direction: (i32, i32),
) {
    let mut tallest = -1;

    let (mut x, mut y) = (start_x, start_y);
    while x < grid[0].len() && y < grid.len() {
        if grid[y][x] > tallest {
            tallest = grid[y][x];
            visible[y][x] = true;
        }
        x = (x as i32 + direction.0) as usize;
        y = (y as i32 + direction.1) as usize;
    }
}

fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
30373
25512
65332
33549
35390";

    #[test]
    fn part1_ex() {
        assert_eq!(21, part1(INPUT));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(8, part2(INPUT));
    }
}
