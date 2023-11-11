use std::collections::VecDeque;

use itertools::Itertools;
use util::grid::Direction;

const INITIAL_PASSWORD: &str = "abcdefgh";
const SCRAMBLED_PASSWORD: &str = "fbgdceah";

struct SwapPosition(usize, usize);
struct SwapLetter(char, char);
struct RotateSteps(Direction, usize);
struct RotatePosition(char);
struct Reverse(usize, usize);
struct Move(usize, usize);

trait Execute {
    fn execute(&self, data: &mut VecDeque<char>);
}

impl Execute for SwapPosition {
    fn execute(&self, data: &mut VecDeque<char>) {
        data.swap(self.0, self.1);
    }
}

impl Execute for SwapLetter {
    fn execute(&self, data: &mut VecDeque<char>) {
        let (x, y) = (self.0, self.1);
        for ch in data.iter_mut() {
            if ch == &x {
                *ch = y;
            } else if ch == &y {
                *ch = x;
            }
        }
    }
}

impl Execute for RotateSteps {
    fn execute(&self, data: &mut VecDeque<char>) {
        match self.0 {
            Direction::Right => data.rotate_right(self.1 % data.len()),
            Direction::Left => data.rotate_left(self.1 % data.len()),
            _ => unreachable!(),
        }
    }
}

impl Execute for RotatePosition {
    fn execute(&self, data: &mut VecDeque<char>) {
        if let Some(index) = position(data, self.0) {
            let rotations = if index < 4 { 1 + index } else { 2 + index };
            data.rotate_right(rotations % data.len());
        }
    }
}

impl Execute for Reverse {
    fn execute(&self, data: &mut VecDeque<char>) {
        let mut low = self.0;
        let mut high = self.1;

        while low < high {
            let (a, b) = (data[low], data[high]);
            data[high] = a;
            data[low] = b;

            low += 1;
            high -= 1;
        }
    }
}

impl Execute for Move {
    fn execute(&self, data: &mut VecDeque<char>) {
        if let Some(ch) = data.remove(self.0) {
            data.insert(self.1, ch);
        }
    }
}

pub fn part1(input: &str) -> String {
    let operations = parse(input);
    scramble_password(INITIAL_PASSWORD, &operations)
}

pub fn part2(input: &str) -> String {
    let operations = parse(input);
    unscramble_password(INITIAL_PASSWORD, SCRAMBLED_PASSWORD, &operations).unwrap()
}

fn scramble_password(initial_password: &str, operations: &[Box<dyn Execute>]) -> String {
    let mut data = raw_password_to_data(initial_password);
    for operation in operations {
        operation.execute(&mut data);
    }

    data_to_string(&data)
}

fn unscramble_password(
    initial_password: &str,
    scrambled: &str,
    operations: &[Box<dyn Execute>],
) -> Option<String> {
    initial_password
        .chars()
        .permutations(initial_password.len())
        .map(|chars| chars.iter().collect::<String>())
        .find(|password| scramble_password(password, operations) == scrambled)
}

fn raw_password_to_data(raw_password: &str) -> VecDeque<char> {
    raw_password.chars().collect()
}

fn data_to_string(data: &VecDeque<char>) -> String {
    data.iter().collect()
}

fn position(data: &VecDeque<char>, searching: char) -> Option<usize> {
    for (i, ch) in data.iter().enumerate() {
        if ch == &searching {
            return Some(i);
        }
    }
    None
}

fn parse(input: &str) -> Vec<Box<dyn Execute>> {
    input
        .lines()
        .map(|line| {
            let parts = line.split_whitespace().collect_vec();
            build_operation(&parts)
        })
        .collect()
}

fn build_operation(parts: &[&str]) -> Box<dyn Execute> {
    match parts[0] {
        "swap" => match parts[1] {
            "position" => {
                let x = parts[2].parse().unwrap();
                let y = parts[5].parse().unwrap();
                Box::new(SwapPosition(x, y))
            }
            "letter" => {
                let x = parts[2].chars().next().unwrap();
                let y = parts[5].chars().next().unwrap();
                Box::new(SwapLetter(x, y))
            }
            _ => unreachable!(),
        },
        "rotate" => match parts[1] {
            "left" => {
                let amount = parts[2].parse().unwrap();
                Box::new(RotateSteps(Direction::Left, amount))
            }
            "right" => {
                let amount = parts[2].parse().unwrap();
                Box::new(RotateSteps(Direction::Right, amount))
            }
            "based" => {
                let ch = parts[6].chars().next().unwrap();
                Box::new(RotatePosition(ch))
            }
            _ => unreachable!(),
        },
        "reverse" => {
            let from = parts[2].parse().unwrap();
            let to = parts[4].parse().unwrap();
            Box::new(Reverse(from, to))
        }
        "move" => {
            let remove = parts[2].parse().unwrap();
            let add_at = parts[5].parse().unwrap();
            Box::new(Move(remove, add_at))
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INITIAL_PASSWORD: &str = "abcde";
    const INPUT: &str = include_str!("../../examples/day21.txt");

    #[test]
    fn swap_position_test() {
        let mut data = raw_password_to_data(INITIAL_PASSWORD);
        let operation = SwapPosition(1, 3);

        operation.execute(&mut data);

        assert_eq!("adcbe", data_to_string(&data));
    }

    #[test]
    fn swap_letter_test() {
        let mut data = raw_password_to_data(INITIAL_PASSWORD);
        let operation = SwapLetter('a', 'c');

        operation.execute(&mut data);

        assert_eq!("cbade", data_to_string(&data));
    }

    #[test]
    fn rotate_steps_right_test() {
        let mut data = raw_password_to_data(INITIAL_PASSWORD);
        let operation = RotateSteps(Direction::Right, 4);

        operation.execute(&mut data);

        assert_eq!("bcdea", data_to_string(&data));
    }

    #[test]
    fn rotate_steps_left_test() {
        let mut data = raw_password_to_data(INITIAL_PASSWORD);
        let operation = RotateSteps(Direction::Left, 2);

        operation.execute(&mut data);

        assert_eq!("cdeab", data_to_string(&data));
    }

    #[test]
    fn rotate_position_test() {
        let mut data = raw_password_to_data(INITIAL_PASSWORD);
        let operation = RotatePosition('b');

        operation.execute(&mut data);

        assert_eq!("deabc", data_to_string(&data));
    }

    #[test]
    fn reverse_test() {
        let mut data = raw_password_to_data(INITIAL_PASSWORD);
        let operation = Reverse(1, 3);

        operation.execute(&mut data);

        assert_eq!("adcbe", data_to_string(&data));
    }

    #[test]
    fn move_test() {
        let mut data = raw_password_to_data(INITIAL_PASSWORD);
        let operation = Move(3, 1);

        operation.execute(&mut data);

        assert_eq!("adbce", data_to_string(&data));
    }

    #[test]
    fn scramble_password_example() {
        let operations = parse(INPUT);
        assert_eq!(
            "decab".to_string(),
            scramble_password(INITIAL_PASSWORD, &operations)
        );
    }
}
