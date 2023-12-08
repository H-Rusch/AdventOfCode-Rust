use num::Integer;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use util::grid::Direction;

struct Node {
    left: String,
    right: String,
}

pub fn part1(input: &str) -> usize {
    let (directions, transitions) = parse(input);
    let goal_state = HashSet::from(["ZZZ"]);

    steps_to_reach("AAA", &goal_state, directions, &transitions)
}

pub fn part2(input: &str) -> usize {
    let (directions, transitions) = parse(input);
    let goal_state = transitions
        .keys()
        .filter(|key| key.ends_with('Z'))
        .map(String::as_str)
        .collect::<HashSet<&str>>();
    
    transitions
        .keys()
        .filter(|key| key.ends_with('A'))
        .map(String::as_str)
        .map(|start| steps_to_reach(start, &goal_state, directions.clone(), &transitions))
        .fold(1, |x, y| x.lcm(&y))
}

fn steps_to_reach(
    start: &str,
    goal_states: &HashSet<&str>,
    directions: Vec<Direction>,
    transitions: &HashMap<String, Node>,
) -> usize {
    let mut steps = 0;
    let mut directions = directions.iter().cycle();
    let mut current = start;

    while !goal_states.contains(current) {
        let direction = directions.next().unwrap();
        current = next_element(current, direction, transitions);

        steps += 1;
    }

    steps
}

fn next_element<'a>(
    state: &'a str,
    direction: &'a Direction,
    transitions: &'a HashMap<String, Node>,
) -> &'a str {
    match direction {
        Direction::Right => &transitions.get(state).unwrap().right,
        Direction::Left => &transitions.get(state).unwrap().left,
        _ => unreachable!(),
    }
}

fn parse(input: &str) -> (Vec<Direction>, HashMap<String, Node>) {
    let transition_pattern = Regex::new(r"(.{3}) = \((.{3}), (.{3})\)").unwrap();
    let (directions, transitions) = input.split_once("\n\n").unwrap();
    let directions = directions.chars().map(Direction::from).collect();

    let nodes = transitions
        .lines()
        .map(|line| {
            let captures = transition_pattern.captures(line).unwrap();
            let key = captures.get(1).unwrap().as_str().to_string();
            let left = captures.get(2).unwrap().as_str().to_string();
            let right = captures.get(3).unwrap().as_str().to_string();

            (key, Node { left, right })
        })
        .collect();

    (directions, nodes)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = include_str!("../../examples/day08_1.txt");
    const EXAMPLE_2: &str = include_str!("../../examples/day08_2.txt");

    #[test]
    fn part1_ex() {
        assert_eq!(6, part1(EXAMPLE_1));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(6, part2(EXAMPLE_2));
    }
}
