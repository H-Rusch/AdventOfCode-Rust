use itertools::Itertools;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};
use util::grid::Coordinate;

struct Pathfinder {
    valid: HashSet<Coordinate>,
    named_coordinates: HashMap<char, Coordinate>,
    distances: HashMap<(char, char), usize>,
    shortest_path_len: usize,
    returning: bool,
}

impl Pathfinder {
    const START: char = '0';

    fn new(
        valid: HashSet<Coordinate>,
        named_coordinates: HashMap<char, Coordinate>,
        returning: bool,
    ) -> Self {
        Pathfinder {
            valid,
            named_coordinates,
            distances: HashMap::new(),
            shortest_path_len: usize::MAX,
            returning,
        }
    }

    fn shortest_path_visiting_all(&mut self) -> usize {
        self.compute_distances();

        let visited = HashSet::from([Pathfinder::START]);
        self.find_shortest_path_len(Pathfinder::START, visited, 0);

        self.shortest_path_len
    }

    fn compute_distances(&mut self) {
        let distances: Vec<(char, char, usize)> = self
            .named_coordinates
            .keys()
            .tuple_combinations()
            .par_bridge()
            .map(|(start, goal)| {
                let start_coordinate = *self.named_coordinates.get(start).unwrap();
                let goal_coordinate = *self.named_coordinates.get(goal).unwrap();
                let distance = self.shortest_path_between(start_coordinate, goal_coordinate);

                (*start, *goal, distance)
            })
            .collect();

        for (ch1, ch2, distance) in distances {
            self.distances.insert((ch1, ch2), distance);
            self.distances.insert((ch2, ch1), distance);
        }
    }

    /// depth first search which prunes branches taking longer than the current shortest path
    fn find_shortest_path_len(
        &mut self,
        current_location: char,
        visited: HashSet<char>,
        current_sum: usize,
    ) {
        if current_sum > self.shortest_path_len {
            return;
        }
        if self.named_coordinates.keys().len() == visited.len() {
            let path_distance = match self.returning {
                true => current_sum + self.get_distance(current_location, Pathfinder::START),
                false => current_sum,
            };
            self.shortest_path_len = self.shortest_path_len.min(path_distance);
        }

        for ch in self.remaining_locations(&visited) {
            let distance = self.get_distance(current_location, ch);
            let mut next_visited = visited.clone();
            next_visited.insert(ch);

            self.find_shortest_path_len(ch, next_visited, current_sum + distance);
        }
    }

    fn get_distance(&self, from: char, to: char) -> usize {
        *self.distances.get(&(from, to)).unwrap()
    }

    fn remaining_locations(&self, visited: &HashSet<char>) -> Vec<char> {
        self.named_coordinates
            .keys()
            .filter(|ch| !visited.contains(ch))
            .copied()
            .collect()
    }

    fn shortest_path_between(&self, start: Coordinate, goal: Coordinate) -> usize {
        let mut queue = VecDeque::from([(0, start)]);
        let mut visited = HashSet::new();

        while let Some((steps, coordinate)) = queue.pop_front() {
            if coordinate == goal {
                return steps;
            }

            if visited.contains(&coordinate) {
                continue;
            }
            visited.insert(coordinate);

            queue.extend(
                coordinate
                    .get_adjacent()
                    .into_iter()
                    .filter(|adjacent| self.valid.contains(adjacent))
                    .map(|adjacent| (steps + 1, adjacent)),
            );
        }

        unreachable!()
    }
}

pub fn part1(input: &str) -> usize {
    let (valid, named_coordinates) = parse(input);
    let mut pathfinder = Pathfinder::new(valid, named_coordinates, false);
    pathfinder.shortest_path_visiting_all()
}

pub fn part2(input: &str) -> usize {
    let (valid, named_coordinates) = parse(input);
    let mut pathfinder = Pathfinder::new(valid, named_coordinates, true);
    pathfinder.shortest_path_visiting_all()
}

fn parse(input: &str) -> (HashSet<Coordinate>, HashMap<char, Coordinate>) {
    let mut valid = HashSet::new();
    let mut named_coordinates = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                continue;
            }
            let coordinate = Coordinate::from(x as i32, y as i32);
            valid.insert(coordinate);
            if ch.is_ascii_digit() {
                named_coordinates.insert(ch, coordinate);
            }
        }
    }

    (valid, named_coordinates)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../examples/day24.txt");

    #[test]
    fn part1_ex() {
        assert_eq!(14, part1(INPUT));
    }
}
