use crate::solutions::computer::{Computer, Operation};

pub fn part1(input: &str) -> i32 {
    let instructions = parse(input);
    let mut computer = Computer::new(instructions);
    computer.insert("a".to_string(), 7);
    computer.run();

    computer.get("a").unwrap()
}

pub fn part2(input: &str) -> i32 {
    /*
    Added support for a multiplication instruction as hinted in the puzzle description.
    A section like:

    cpy b c
    inc a
    dec c
    jnz c -2
    dec d
    jnz d -5

    can now be replaced with:
    mul a d b
    cpy 0 c
    cpy 0 d
    noop
    noop
    noop

    This only works when no toggle instruction is executed on that block of instructions.
    */
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
