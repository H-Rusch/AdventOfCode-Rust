use std::collections::HashSet;
use util::grid::{Coordinate, Direction};

pub fn part1(input: &str) -> u32 {
    let mut direction = Direction::UP;
    let mut coordinate = Coordinate::default();

    for (turn, steps) in parse(input) {
        change_direction(&mut direction, turn);
        coordinate.step(&direction, steps);
    }

    coordinate.manhatten_distance(&Coordinate::default())
}

pub fn part2(input: &str) -> u32 {
    let mut direction = Direction::RIGHT;
    let mut coordinate = Coordinate::default();
    let mut visited = HashSet::new();

    'outer: for (turn, steps) in parse(input) {
        change_direction(&mut direction, turn);

        for _ in 1..=steps {
            coordinate.step(&direction, 1);
            if visited.contains(&coordinate) {
                break 'outer;
            }
            visited.insert(coordinate.clone());
        }
    }

    coordinate.manhatten_distance(&Coordinate::default())
}

fn change_direction(direction: &mut Direction, turn: char) {
    *direction = match turn {
        'R' => direction.turn_right(),
        'L' => direction.turn_left(),
        _ => unreachable!(),
    };
}

fn parse(input: &str) -> impl Iterator<Item = (char, u32)> + '_ {
    input.split(", ").map(|line| {
        let mut chars = line.chars();
        chars
            .next()
            .map(|c| (c, chars.as_str().parse().unwrap()))
            .unwrap()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex() {
        assert_eq!(5, part1("R2, L3"));
        assert_eq!(2, part1("R2, R2, R2"));
        assert_eq!(12, part1("R5, L5, R5, R3"));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(4, part2("R8, R4, R4, R8"));
    }
}
