use crate::solutions::computer::{Computer, Operation};

pub fn part1(input: &str) -> i32 {
    let instructions = parse(input);
    let mut computer = Computer::new(instructions);
    computer.insert("a".to_string(), 7);
    computer.run();

    computer.get("a").unwrap()
}

pub fn part2(input: &str) -> i32 {
    let instructions = parse(input);
    let mut computer = Computer::new(instructions);
    computer.insert("a".to_string(), 12);
    computer.run();

    computer.get("a").unwrap()
}

fn parse(input: &str) -> Vec<Operation> {
    input.lines().map(Operation::from).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../examples/day23.txt");

    #[test]
    fn part1_ex() {
        assert_eq!(3, part1(INPUT));
    }
}
