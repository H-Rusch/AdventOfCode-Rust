use std::collections::VecDeque;

use itertools::Itertools;
use lazy_static::lazy_static;
use util::grid::{Bounds, Coordinate, Direction};

lazy_static!(
    static ref GOAL: Coordinate = Coordinate::from(3, 3);
    static ref BOUNDS: Bounds = Bounds::from(0..4, 0..4);
);

pub fn part1(input: &str) -> String {
    find_shortest_path(*GOAL, input, &BOUNDS)
}

pub fn part2(input: &str) -> usize {
    find_longest_path(*GOAL, input, &BOUNDS).len()
}

fn find_shortest_path(goal: Coordinate, passcode: &str, bounds: &Bounds) -> String {
    let mut queue = VecDeque::from([(Coordinate::default(), "".to_string())]);
    while let Some((coordinate, path)) = queue.pop_front() {
        if coordinate == goal {
            return path;
        }

        for (coordinate, next_direction_char) in
            get_valid_adjacent(&coordinate, bounds, passcode, &path)
        {
            queue.push_back((coordinate, format!("{path}{next_direction_char}")));
        }
    }

    unreachable!()
}

fn find_longest_path(goal: Coordinate, passcode: &str, bounds: &Bounds) -> String {
    let mut queue = VecDeque::from([(Coordinate::default(), "".to_string())]);
    let mut longest_path = String::new();

    while let Some((coordinate, path)) = queue.pop_front() {
        if coordinate == goal {
            longest_path = path.clone();
            continue;
        }

        for (coordinate, next_direction_char) in
            get_valid_adjacent(&coordinate, bounds, passcode, &path)
        {
            queue.push_back((coordinate, format!("{path}{next_direction_char}")));
        }
    }

    longest_path
}

fn get_valid_adjacent(
    coordinate: &Coordinate,
    bounds: &Bounds,
    passcode: &str,
    path: &str,
) -> Vec<(Coordinate, char)> {
    let mut directions = vec![
        (Direction::Right, 'R'),
        (Direction::Up, 'U'),
        (Direction::Left, 'L'),
        (Direction::Down, 'D'),
    ];

    remove_locked_directions(&mut directions, passcode, path);
    transform_to_valid_coordinates(&directions, coordinate, bounds)
}

fn remove_locked_directions(directions: &mut Vec<(Direction, char)>, passcode: &str, path: &str) {
    let to_hash = format!("{passcode}{path}");
    let locks = hash(&to_hash).chars().take(4).collect_vec();
    let open = 'b'..='f';

    directions.retain(|(direction, _)| match direction {
        Direction::Up => open.contains(&locks[0]),
        Direction::Down => open.contains(&locks[1]),
        Direction::Left => open.contains(&locks[2]),
        Direction::Right => open.contains(&locks[3]),
    });
}

fn transform_to_valid_coordinates(
    directions: &[(Direction, char)],
    coordinate: &Coordinate,
    bounds: &Bounds,
) -> Vec<(Coordinate, char)> {
    directions
        .iter()
        .map(|(direction, ch)| (coordinate.step_in_bounds(direction, 1, bounds), ch))
        .filter(|(coordinate, _)| coordinate.is_some())
        .map(|(coordinate, ch)| (coordinate.unwrap(), *ch))
        .collect()
}

fn hash(to_hash: &str) -> String {
    let digest = md5::compute(to_hash);
    format!("{:x}", digest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex() {
        assert_eq!("DDRRRD".to_string(), part1("ihgpwlah"));
        assert_eq!("DDUDRLRRUDRD".to_string(), part1("kglvqrro"));
        assert_eq!(
            "DRURDRUDDLLDLUURRDULRLDUUDDDRR".to_string(),
            part1("ulqzkmiv")
        );
    }

    #[test]
    fn part2_ex() {
        assert_eq!(370, part2("ihgpwlah"));
        assert_eq!(492, part2("kglvqrro"));
        assert_eq!(830, part2("ulqzkmiv"));
    }
}
