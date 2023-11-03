use util::grid::{Bounds, Coordinate, Direction};

const KEYPAD_1: [[char; 3]; 3] = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];

const KEYPAD_2: [[char; 5]; 5] = [
    [' ', ' ', '1', ' ', ' '],
    [' ', '2', '3', '4', ' '],
    ['5', '6', '7', '8', '9'],
    [' ', 'A', 'B', 'C', ' '],
    [' ', ' ', 'D', ' ', ' '],
];

pub fn part1(input: &str) -> String {
    let coordinate = Coordinate::from(1, 1);
    let bounds = Bounds::from(0..KEYPAD_1[0].len() as i32, 0..KEYPAD_1.len() as i32);
    let keypad = KEYPAD_1.iter().map(|&line| line.to_vec()).collect();

    build_code(keypad, input, coordinate, &bounds)
}

pub fn part2(input: &str) -> String {
    let coordinate = Coordinate::from(0, 2);
    let bounds = Bounds::from(0..KEYPAD_2[0].len() as i32, 0..KEYPAD_2.len() as i32);
    let keypad = KEYPAD_2.iter().map(|&line| line.to_vec()).collect();

    build_code(keypad, input, coordinate, &bounds)
}

fn build_code(
    keypad: Vec<Vec<char>>,
    input: &str,
    mut coordinate: Coordinate,
    bounds: &Bounds,
) -> String {
    parse(input)
        .iter()
        .map(|line| {
            coordinate = execute_line(&keypad, line, coordinate, &bounds);
            let (x, y) = (coordinate.x as usize, coordinate.y as usize);
            keypad[y][x]
        })
        .collect()
}

fn execute_line(
    keypad: &Vec<Vec<char>>,
    line: &Vec<Direction>,
    mut coordinate: Coordinate,
    bounds: &Bounds,
) -> Coordinate {
    for direction in line.iter() {
        if let Some(next) = coordinate.step_in_bounds(direction, 1, &bounds) {
            let (x, y) = (next.x as usize, next.y as usize);
            if keypad[y][x] != ' ' {
                coordinate = next;
            }
        }
    }

    coordinate
}

fn parse(input: &str) -> Vec<Vec<Direction>> {
    input
        .lines()
        .map(|line| line.chars().map(Direction::from).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../examples/day02.txt");

    #[test]
    fn part1_ex() {
        assert_eq!("1985", part1(EXAMPLE));
    }

    #[test]
    fn part2_ex() {
        assert_eq!("5DB3", part2(EXAMPLE));
    }
}
