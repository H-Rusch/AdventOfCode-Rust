use crate::solutions::computer::{Computer, Operation};

pub fn part1(input: &str) -> i32 {
    let instructions = parse(input);
    let mut computer = Computer::new(instructions);
    computer.run();

    computer.get("a").unwrap()
}

pub fn part2(input: &str) -> i32 {
    let instructions = parse(input);
    let mut computer = Computer::new(instructions);
    computer.insert("c".to_string(), 1);
    computer.run();

    computer.get("a").unwrap()
}

fn parse(input: &str) -> Vec<Operation> {
    input.lines().map(Operation::from).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../examples/day12.txt");

    #[test]
    fn part1_ex() {
        assert_eq!(42, part1(INPUT));
    }
}
