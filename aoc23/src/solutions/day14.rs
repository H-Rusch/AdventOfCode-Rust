use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use util::grid::{Bounds, Coordinate, Direction};

const LIMIT: usize = 1_000_000_000;

#[derive(Clone)]
struct Board {
    bounds: Bounds,
    stones: HashSet<Coordinate>,
    walls: HashSet<Coordinate>,
}

trait StoneSortKey {
    fn get_sort_key(&self) -> Box<dyn FnMut(&&Coordinate) -> i32>;
}

impl StoneSortKey for Direction {
    fn get_sort_key(&self) -> Box<dyn FnMut(&&Coordinate) -> i32> {
        let closure = match *self {
            Direction::Up => |coordinate: &&Coordinate| coordinate.y,
            Direction::Down => |coordinate: &&Coordinate| -coordinate.y,
            Direction::Left => |coordinate: &&Coordinate| coordinate.x,
            Direction::Right => |coordinate: &&Coordinate| -coordinate.x,
        };

        Box::new(closure)
    }
}

impl Board {
    fn tilt(&mut self, direction: &Direction) {
        let mut next_stones: HashSet<Coordinate> = HashSet::new();

        // sort first -> e.g. when tilting up stones at the top have to roll first
        for stone in self.sort_stones(direction) {
            next_stones.insert(self.roll_stone(*stone, direction, &next_stones));
        }

        self.stones = next_stones;
    }

    fn sort_stones(&self, direction: &Direction) -> impl Iterator<Item = &Coordinate> {
        self.stones.iter().sorted_by_key(direction.get_sort_key())
    }

    fn roll_stone(
        &self,
        stone: Coordinate,
        direction: &Direction,
        settled_stones: &HashSet<Coordinate>,
    ) -> Coordinate {
        let mut stone = stone;
        while let Some(shifted_stone) = stone.step_in_bounds(direction, 1, &self.bounds) {
            if settled_stones.contains(&shifted_stone) || self.walls.contains(&shifted_stone) {
                break;
            }
            stone = shifted_stone;
        }

        stone
    }

    fn calculate_load(&self) -> usize {
        self.stones
            .iter()
            .map(|stone| self.bounds.height() - stone.y as usize)
            .sum()
    }

    fn get_board_representation(&self) -> String {
        self.bounds
            .coordinates()
            .map(|coordinate| {
                if self.walls.contains(&coordinate) {
                    '#'
                } else if self.stones.contains(&coordinate) {
                    'O'
                } else {
                    '.'
                }
            })
            .collect()
    }
}

pub fn part1(input: &str) -> usize {
    let mut board = parse(input);
    board.tilt(&Direction::Up);
    board.calculate_load()
}

pub fn part2(input: &str) -> usize {
    let mut board = parse(input);

    let mut board_memory = HashMap::new();
    let mut value_memory = Vec::new();
    for i in 0..LIMIT {
        let board_state = board.get_board_representation();

        value_memory.push(board.calculate_load());
        if let Some(start) = board_memory.insert(board_state, i) {
            let period = i - start;
            let final_index = start + (LIMIT - start) % period;

            return value_memory[final_index];
        }

        [
            Direction::Up,
            Direction::Left,
            Direction::Down,
            Direction::Right,
        ]
        .iter()
        .for_each(|direction| board.tilt(direction));
    }

    board.calculate_load()
}

fn parse(input: &str) -> Board {
    let mut stones = HashSet::new();
    let mut walls = HashSet::new();

    for (y, row) in input.lines().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            if ch == 'O' {
                stones.insert(Coordinate::from(x as i32, y as i32));
            } else if ch == '#' {
                walls.insert(Coordinate::from(x as i32, y as i32));
            }
        }
    }

    let bounds = Bounds::from(
        0..input.lines().next().unwrap().len() as i32,
        0..input.lines().count() as i32,
    );

    Board {
        bounds,
        stones,
        walls,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../examples/day14.txt");

    #[test]
    fn part1_ex() {
        assert_eq!(136, part1(EXAMPLE));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(64, part2(EXAMPLE));
    }
}
