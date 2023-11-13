use crate::solutions::computer::{Computer, Operation};

pub fn part1(input: &str) -> i32 {
    let instructions = parse(input);

    let mut i = 0;
    loop {
        let mut computer = Computer::new(instructions.clone());
        computer.insert("a".to_string(), i);

        if computer.run_to_output_length(1_000) {
            break;
        }
        i += 1;
    }

    i
}

pub fn part2(_: &str) -> &str {
    "The Easter Bunny 🐇 has been defeated. Christmas wins 🎅"
}

fn parse(input: &str) -> Vec<Operation> {
    input.lines().map(Operation::from).collect()
}
