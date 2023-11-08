use std::collections::{HashSet, VecDeque};
use util::grid::Coordinate;

struct WallCalculator {
    value: i32,
}

impl WallCalculator {
    fn new(value: i32) -> Self {
        WallCalculator { value }
    }

    fn calculate(&self, coordinate: &Coordinate) -> bool {
        let (x, y) = (coordinate.x, coordinate.y);
        let mut number = x * x + 3 * x + 2 * x * y + y + y * y;
        number += self.value;

        number.count_ones() % 2 == 1
    }
}

pub fn part1(input: &str) -> usize {
    let value = input.parse().unwrap();
    let walls = WallCalculator::new(value);

    find_path(Coordinate::from(31, 39), walls)
}

pub fn part2(input: &str) -> usize {
    let value = input.parse().unwrap();
    let walls = WallCalculator::new(value);

    expand_up_to_steps(walls, 50)
}

fn find_path(goal: Coordinate, walls: WallCalculator) -> usize {
    let mut queue = VecDeque::from([(0, Coordinate::from(1, 1))]);
    let mut visited = HashSet::new();

    while let Some((step, coordinate)) = queue.pop_front() {
        if coordinate == goal {
            return step;
        }

        if visited.contains(&coordinate) {
            continue;
        }
        visited.insert(coordinate);

        for adjacent in coordinate.get_adjacent() {
            if adjacent.x < 0 || adjacent.y < 0 || visited.contains(&adjacent) {
                continue;
            }
            if !walls.calculate(&adjacent) {
                queue.push_back((step + 1, adjacent));
            }
        }
    }

    unreachable!()
}

fn expand_up_to_steps(walls: WallCalculator, limit: usize) -> usize {
    let mut queue = VecDeque::from([(0, Coordinate::from(1, 1))]);
    let mut visited = HashSet::new();

    while let Some((step, coordinate)) = queue.pop_front() {
        if visited.contains(&coordinate) || step > limit {
            continue;
        }
        visited.insert(coordinate);

        for adjacent in coordinate.get_adjacent() {
            if adjacent.x < 0 || adjacent.y < 0 || visited.contains(&adjacent) {
                continue;
            }
            if !walls.calculate(&adjacent) {
                queue.push_back((step + 1, adjacent));
            }
        }
    }

    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: i32 = 10;

    #[test]
    fn wall_calculation_works_correctly() {
        let walls = WallCalculator::new(INPUT);

        assert!(!walls.calculate(&Coordinate::from(0, 0)));
        assert!(walls.calculate(&Coordinate::from(1, 0)));
        assert!(!walls.calculate(&Coordinate::from(2, 0)));
        assert!(!walls.calculate(&Coordinate::from(0, 1)));
        assert!(!walls.calculate(&Coordinate::from(1, 1)));
        assert!(walls.calculate(&Coordinate::from(2, 1)));
        assert!(walls.calculate(&Coordinate::from(0, 2)));
        assert!(!walls.calculate(&Coordinate::from(1, 2)));
        assert!(!walls.calculate(&Coordinate::from(2, 2)));
    }

    #[test]
    fn part1_ex() {
        let walls = WallCalculator::new(INPUT);

        assert_eq!(11, find_path(Coordinate::from(7, 4), walls));
    }
}
