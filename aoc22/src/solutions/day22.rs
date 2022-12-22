use std::collections::HashSet;

#[derive(Debug)]
enum Instr {
    Walk(u32),
    TurnLeft,
    TurnRight,
}

#[derive(Debug)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

impl Direction {
    fn turn_right(&mut self) {
        match self {
            Direction::Up => *self = Direction::Right,
            Direction::Right => *self = Direction::Down,
            Direction::Down => *self = Direction::Left,
            Direction::Left => *self = Direction::Up,
        }
    }

    fn turn_left(&mut self) {
        match self {
            Direction::Up => *self = Direction::Left,
            Direction::Left => *self = Direction::Down,
            Direction::Down => *self = Direction::Right,
            Direction::Right => *self = Direction::Up,
        }
    }

    fn get_score(&self) -> i32 {
        match self {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }
}

pub fn part1(input: &str) -> i32 {
    let ((open_map, wall_map), instructions) = parse(input);

    let all_tiles = open_map.union(&wall_map).collect();

    // start in left most coordinate in the top row, facing right
    let mut position = *open_map.iter().filter(|(_, y)| *y == 0).min().unwrap();
    let mut direction = Direction::Right;

    for instruction in instructions {
        match instruction {
            Instr::TurnRight => direction.turn_right(),
            Instr::TurnLeft => direction.turn_left(),
            Instr::Walk(n) => {
                position = take_steps(position, &direction, n, &open_map, &wall_map, &all_tiles);
            }
        }
    }

    // row and column counts start at 1
    (position.1 + 1) * 1000 + (position.0 + 1) * 4 + direction.get_score()
}

fn take_steps(
    position: (i32, i32),
    direction: &Direction,
    n: u32,
    open: &HashSet<(i32, i32)>,
    walls: &HashSet<(i32, i32)>,
    all_tiles: &HashSet<&(i32, i32)>,
) -> (i32, i32) {
    let mut current_position = position;
    for _ in 0..n {
        let (x, y) = current_position;
        let next_position = match direction {
            Direction::Right => (x + 1, y),
            Direction::Up => (x, y - 1),
            Direction::Left => (x - 1, y),
            Direction::Down => (x, y + 1),
        };

        // early returns if the next position is on the map or if it is blocked by a wall
        if open.contains(&next_position) {
            current_position = next_position;
            continue;
        }
        if walls.contains(&next_position) {
            break;
        }

        // next position is not in the open spaces and also not blocked -> wrap around and then chack if that space is blocked by a wall
        let next_position = match direction {
            Direction::Right => **all_tiles.iter().filter(|(_, yy)| *yy == y).min().unwrap(),
            Direction::Up => **all_tiles.iter().filter(|(xx, _)| *xx == x).max().unwrap(),
            Direction::Left => **all_tiles.iter().filter(|(_, yy)| *yy == y).max().unwrap(),
            Direction::Down => **all_tiles.iter().filter(|(xx, _)| *xx == x).min().unwrap(),
        };

        if walls.contains(&next_position) {
            break;
        }

        current_position = next_position;
        // current_position = next_tile(current_position, direction, open, walls);
    }

    current_position
}

pub fn part2(_input: &str) -> usize {
    0
}

type TileMaps = (HashSet<(i32, i32)>, HashSet<(i32, i32)>);

fn parse(input: &str) -> (TileMaps, Vec<Instr>) {
    let mut open_tiles = HashSet::new();
    let mut wall_tiles = HashSet::new();

    let (layout, path) = input.split_once("\n\n").unwrap();
    for (y, row) in layout.lines().enumerate() {
        for (x, cell) in row.chars().enumerate() {
            let (x, y) = (x as i32, y as i32);
            match cell {
                '.' => {
                    open_tiles.insert((x, y));
                }
                '#' => {
                    wall_tiles.insert((x, y));
                }
                _ => {}
            }
        }
    }

    let mut instructions = Vec::new();
    let mut val = 0;
    for c in path.trim().chars() {
        if c.is_ascii_digit() {
            val = val * 10 + c.to_digit(10).unwrap();
        } else {
            instructions.push(Instr::Walk(val));
            val = 0;

            let turn = match c {
                'R' => Instr::TurnRight,
                'L' => Instr::TurnLeft,
                _ => unreachable!(),
            };
            instructions.push(turn);
        }
    }
    instructions.push(Instr::Walk(val));

    ((open_tiles, wall_tiles), instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../examples/day22.txt");

    #[test]
    fn part1_ex() {
        assert_eq!(6032, part1(INPUT));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(0, part2(INPUT));
    }
}
