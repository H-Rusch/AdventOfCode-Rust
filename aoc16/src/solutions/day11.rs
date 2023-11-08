use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};

use itertools::Itertools;
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
struct State {
    level: usize,
    floors: Vec<HashSet<i32>>,
}

impl State {
    fn from(level: usize, floors: Vec<HashSet<i32>>) -> Self {
        State { level, floors }
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.level.hash(state);
        for (i, floor_items) in self.floors.iter().enumerate() {
            let mut floor_items = floor_items.iter().collect_vec();
            floor_items.sort();
            for value in floor_items.iter() {
                (i, **value).hash(state);
            }
        }
    }
}

pub fn part1(input: &str) -> usize {
    let floors = parse(input);
    let goal = generate_goal(&floors);

    solve(floors, goal)
}

pub fn part2(input: &str) -> usize {
    let mut floors = parse(input);
    insert_additional_values(&mut floors);
    let goal = generate_goal(&floors);

    solve(floors, goal)
}

fn generate_goal(floors: &[HashSet<i32>]) -> HashSet<i32> {
    floors.iter().flatten().map(|i| *i).collect()
}

fn solve(floors: Vec<HashSet<i32>>, goal: HashSet<i32>) -> usize {
    let mut queue = VecDeque::from([(0, State::from(0, floors))]);
    let mut visited = HashSet::new();

    while let Some((step, state)) = queue.pop_front() {
        //println!("{:?}", state.floors);
        if state.floors[state.floors.len() - 1] == goal {
            return step;
        }

        if visited.contains(&(state)) {
            continue;
        }

        for next_state in next_states(&state) {
            if !visited.contains(&next_state) {
                queue.push_back((step + 1, next_state));
            }
        }
        visited.insert(state);
    }

    unreachable!()
}

fn next_states(state: &State) -> Vec<State> {
    let level = state.level;
    let mut result = Vec::new();
    let all_combinations = state.floors[level]
        .iter()
        .combinations(1)
        .chain(state.floors[level].iter().combinations(2));

    for taken_items in all_combinations {
        let taken_items: HashSet<i32> = HashSet::from_iter(taken_items.iter().map(|i| **i));

        if level != 0 {
            let next_level = level - 1;
            if let Some(next_floors) = build_next_floors(&state, next_level, &taken_items) {
                result.push(State::from(next_level, next_floors));
            }
        }
        if level != state.floors.len() - 1 {
            let next_level = level + 1;
            if let Some(next_floors) = build_next_floors(&state, next_level, &taken_items) {
                result.push(State::from(next_level, next_floors));
            }
        }
    }

    result
}

fn build_next_floors(
    state: &State,
    next_level: usize,
    taken_items: &HashSet<i32>,
) -> Option<Vec<HashSet<i32>>> {
    let mut new_floors = state.floors.clone().to_owned();
    new_floors[state.level].retain(|i| !taken_items.contains(i));
    new_floors[next_level].extend(taken_items.iter());

    if is_valid_floor(&new_floors[state.level]) && is_valid_floor(&new_floors[next_level]) {
        return Some(new_floors);
    }
    None
}

fn is_valid_floor(floor: &HashSet<i32>) -> bool {
    floor.iter().all(|i| *i > 0)
        || floor.iter().all(|i| *i < 0)
        // for all microchips there is the associated generator on the same floor
        || floor
            .iter()
            .filter(|i| **i > 0)
            .all(|i| floor.contains(&-i))
}

fn parse(input: &str) -> Vec<HashSet<i32>> {
    let mut floors = vec![HashSet::new(); input.lines().count()];
    let mut resource_map = HashMap::new();
    let mut resource_num = 1;

    let chip_pattern = Regex::new(r"(\w+)-compatible microchip").unwrap();
    for (i, line) in input.lines().enumerate() {
        for capture in chip_pattern.captures_iter(line) {
            let resource = capture.get(1).unwrap().as_str();
            resource_map.insert(resource, resource_num);
            floors[i].insert(resource_num);
            resource_num += 1;
        }
    }

    let generator_pattern = Regex::new(r"(\w+) generator").unwrap();
    for (i, line) in input.lines().enumerate() {
        for capture in generator_pattern.captures_iter(line) {
            let resource = capture.get(1).unwrap().as_str();
            floors[i].insert(-1 * resource_map.get(resource).unwrap());
        }
    }

    floors
}

fn insert_additional_values(floors: &mut [HashSet<i32>]) {
    let all_values = floors.iter().flatten();
    let max_value = all_values.max().unwrap();

    for val in [max_value + 1, max_value + 2].iter() {
        floors[0].insert(*val);
        floors[0].insert(-val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../examples/day11.txt");

    #[test]
    fn state_hash_test() {
        let mut set = HashSet::new();
        set.insert(State::from(
            0,
            vec![HashSet::new(), HashSet::from([1, -1, 2, -2])],
        ));
        assert!(set.contains(&State::from(
            0,
            vec![HashSet::new(), HashSet::from([1, -1, 2, -2])]
        )));
    }

    #[test]
    fn is_valid_floor_test() {
        assert!(is_valid_floor(&HashSet::from([-1])));
        assert!(is_valid_floor(&HashSet::from([1])));
        assert!(is_valid_floor(&HashSet::from([-1, 1])));
        assert!(is_valid_floor(&HashSet::from([-1, 1, -2])));
        assert!(is_valid_floor(&HashSet::from([-1, -2])));
        assert!(is_valid_floor(&HashSet::from([1, 2])));
        assert!(is_valid_floor(&HashSet::from([1, -1, 2, -2])));
        assert!(!is_valid_floor(&HashSet::from([1, -2])));
    }

    #[test]
    fn goal_state_recognized() {
        let floors = vec![HashSet::new(), HashSet::from([1, -1, 2, -2])];
        let goal = generate_goal(&floors);
        assert_eq!(goal, floors[1]);
        assert_eq!(0, solve(floors, goal));
    }

    #[test]
    fn part1_ex() {
        assert_eq!(11, part1(INPUT));
    }
}
