use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

pub fn part1(input: &str) -> u32 {
    let grid = parse(input);

    let (start_x, start_y) = find_char(&grid, 'S');

    find_shortest_path(start_x, start_y, &grid, 'E', true)
}

pub fn part2(input: &str) -> u32 {
    // go backwards from end to first 'a'
    let grid = parse(input);

    let (start_x, start_y) = find_char(&grid, 'E');

    find_shortest_path(start_x, start_y, &grid, 'a', false)
}

fn find_shortest_path(x: usize, y: usize, grid: &[Vec<char>], goal: char, forwards: bool) -> u32 {
    // Breadth-first search
    // state is ((x, y), height, cost)
    let starting_state = ((x, y), calculate_height(grid[y][x]), 0);
    let mut expanded: VecDeque<((usize, usize), u8, u32)> = VecDeque::from([starting_state]);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    while let Some(((x, y), height, cost)) = expanded.pop_front() {
        if visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));

        if grid[y][x] == goal {
            return cost;
        }

        for (xx, yy) in util::get_adjacent(x, y, grid[0].len(), grid.len()) {
            if !visited.contains(&(xx, yy)) {
                let adjacent_char = grid[yy][xx];
                // forwards flag to differentiate between part 1 and part 2
                if forwards {
                    if calculate_height(adjacent_char) - 1 <= height {
                        expanded.push_back(((xx, yy), calculate_height(adjacent_char), cost + 1));
                    }
                } else if calculate_height(adjacent_char) >= height - 1 {
                    expanded.push_back(((xx, yy), calculate_height(adjacent_char), cost + 1));
                }
            }
        }
    }

    unreachable!()
}

fn find_char(grid: &[Vec<char>], search: char) -> (usize, usize) {
    (0..grid[0].len())
        .cartesian_product(0..grid.len())
        .find(|(x, y)| grid[*y][*x] == search)
        .unwrap()
}

fn calculate_height(c: char) -> u8 {
    match c {
        'a'..='z' => c as u8 + 1 - 97,
        'S' => 1,
        'E' => 26,
        _ => unreachable!(),
    }
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../examples/day12.txt");

    #[test]
    fn part1_ex() {
        assert_eq!(31, part1(INPUT));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(29, part2(INPUT));
    }
}
