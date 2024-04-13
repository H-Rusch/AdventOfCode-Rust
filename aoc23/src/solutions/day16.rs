use rayon::iter::{ParallelBridge, ParallelIterator};
use std::collections::{HashMap, HashSet, VecDeque};
use util::grid::{Coordinate, Direction};

pub fn part1(input: &str) -> usize {
    let mirrors = parse(input);
    let energy_levels = simulate_light(&mirrors, Coordinate::from(0, 0), Direction::Right);

    energy_levels.len()
}

pub fn part2(input: &str) -> usize {
    let x_max = input.lines().next().unwrap().len();
    let y_max = input.lines().count();

    let mirrors = parse(input);

    let top_and_bottom = (0..x_max).flat_map(|x| [(x, 0), (x, y_max)]);
    let left_and_right = (0..y_max).flat_map(|y| [(0, y), (x_max, y)]);

    top_and_bottom
        .chain(left_and_right)
        .flat_map(|xy| Direction::iter().map(move |direction| (xy, direction)))
        .map(|((x, y), direction)| (Coordinate::from(x as i32, y as i32), direction))
        .par_bridge()
        .map(|(coordinate, direction)| simulate_light(&mirrors, coordinate, direction))
        .map(|energy_levels| energy_levels.len())
        .max()
        .unwrap()
}

fn simulate_light(
    mirrors: &HashMap<Coordinate, char>,
    start_coodinate: Coordinate,
    start_direction: Direction,
) -> HashSet<Coordinate> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::from([(start_coodinate, start_direction)]);

    while let Some((coodinate, direction)) = queue.pop_front() {
        if !visited.insert((coodinate, direction)) {
            continue;
        }

        if let Some(tile) = mirrors.get(&coodinate) {
            queue.extend(
                change_direction(&direction, *tile)
                    .iter()
                    .map(|next_direction| (coodinate.step(next_direction, 1), *next_direction))
                    .filter(|(next_coodinate, _)| mirrors.contains_key(next_coodinate)),
            );
        }
    }

    visited.iter().map(|(coordinate, _)| *coordinate).collect()
}

fn change_direction(direction: &Direction, tile: char) -> Vec<Direction> {
    match tile {
        '/' => vec![match direction {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Left => Direction::Down,
        }],
        '\\' => vec![match direction {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Left => Direction::Up,
        }],
        '-' => match direction {
            Direction::Left | Direction::Right => vec![*direction],
            Direction::Up | Direction::Down => vec![Direction::Right, Direction::Left],
        },
        '|' => match direction {
            Direction::Left | Direction::Right => vec![Direction::Up, Direction::Down],
            Direction::Up | Direction::Down => vec![*direction],
        },
        '.' => vec![*direction],
        _ => unreachable!(),
    }
}

fn parse(input: &str) -> HashMap<Coordinate, char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices()
                .map(move |(x, ch)| (Coordinate::from(x as i32, y as i32), ch))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../examples/day16.txt");

    #[test]
    fn part1_ex() {
        assert_eq!(46, part1(EXAMPLE));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(51, part2(EXAMPLE));
    }
}
