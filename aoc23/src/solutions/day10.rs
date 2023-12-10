use std::collections::{HashMap, VecDeque};

use util::grid::{Coordinate, Direction};

pub fn part1(input: &str) -> usize {
    let (coordinates, start) = parse(input);
    let circle = find_circle(start, &coordinates);

    circle.len() / 2
}

pub fn part2(input: &str) -> usize {
    0
}

fn find_circle(start: Coordinate, coordinates: &HashMap<Coordinate, char>) -> String {
    let mut queue = init_queue(&start, coordinates);

    while let Some((coordinate, direction, path)) = queue.pop_front() {
        if coordinate == start {
            return path;
        }

        let pipe = coordinates.get(&coordinate).unwrap();
        let next_direction = next_direction(*pipe, direction);
        let next_coordinate = coordinate.step(&next_direction, 1);
        let mut next_path = path.clone();
        next_path.push(*pipe);

        queue.push_back((next_coordinate, next_direction, next_path));
    }

    unreachable!()
}

fn init_queue(
    start: &Coordinate,
    coordinates: &HashMap<Coordinate, char>,
) -> VecDeque<(Coordinate, Direction, String)> {
    [
        Direction::Up,
        Direction::Left,
        Direction::Down,
        Direction::Right,
    ]
    .into_iter()
    .map(|direction| (start.step(&direction, 1), direction))
    .filter(|(coordinate, direction)| {
        coordinates.contains_key(coordinate)
            && valid_directions(*coordinates.get(coordinate).unwrap(), direction)
    })
    .map(|(coordinate, direction)| (coordinate, direction, String::from("S")))
    .collect()
}

fn next_direction(pipe: char, direction: Direction) -> Direction {
    match (pipe, direction) {
        ('|', Direction::Up) => Direction::Up,
        ('|', Direction::Down) => Direction::Down,
        ('-', Direction::Left) => Direction::Left,
        ('-', Direction::Right) => Direction::Right,
        ('L', Direction::Down) => Direction::Right,
        ('L', Direction::Left) => Direction::Up,
        ('J', Direction::Down) => Direction::Left,
        ('J', Direction::Right) => Direction::Up,
        ('7', Direction::Up) => Direction::Left,
        ('7', Direction::Right) => Direction::Down,
        ('F', Direction::Up) => Direction::Right,
        ('F', Direction::Left) => Direction::Down,
        _ => unreachable!(),
    }
}

fn valid_directions(pipe: char, direction: &Direction) -> bool {
    matches!(
        (pipe, direction),
        ('|', Direction::Up | Direction::Down)
            | ('-', Direction::Left | Direction::Right)
            | ('L', Direction::Left | Direction::Down)
            | ('J', Direction::Right | Direction::Down)
            | ('7', Direction::Right | Direction::Up)
            | ('F', Direction::Left | Direction::Up)
            | ('S', _)
    )
}

fn parse(input: &str) -> (HashMap<Coordinate, char>, Coordinate) {
    let mut coordinates = HashMap::new();
    for (y, row) in input.lines().enumerate() {
        for (x, cell) in row.chars().enumerate() {
            if cell != '.' {
                coordinates.insert(Coordinate::from(x as i32, y as i32), cell);
            }
        }
    }

    let start_position = find_start(input);

    (coordinates, start_position)
}

fn find_start(input: &str) -> Coordinate {
    for (y, row) in input.lines().enumerate() {
        for (x, cell) in row.chars().enumerate() {
            if cell == 'S' {
                return Coordinate::from(x as i32, y as i32);
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = include_str!("../../examples/day10_1_1.txt");
    const EXAMPLE_2: &str = include_str!("../../examples/day10_1_2.txt");

    #[test]
    fn part1_ex() {
        assert_eq!(4, part1(EXAMPLE_1));
        assert_eq!(8, part1(EXAMPLE_2));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(0, part2(""));
    }
}
