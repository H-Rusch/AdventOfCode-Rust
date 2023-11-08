use std::collections::HashMap;

use itertools::Itertools;

struct Computer {
    instructions: Vec<Operation>,
    registers: HashMap<String, i32>,
    ic: usize,
}

impl Computer {
    fn new(instructions: Vec<Operation>) -> Self {
        Computer {
            instructions,
            registers: HashMap::new(),
            ic: 0,
        }
    }

    fn run(&mut self) {
        while let Some(instruction) = self.instructions.get(self.ic) {
            let offset = self.execute_instruction(&instruction.clone());
            if offset < 0 {
                self.ic -= offset.unsigned_abs() as usize;
            } else {
                self.ic += offset as usize;
            }
        }
    }

    fn execute_instruction(&mut self, instruction: &Operation) -> i32 {
        match instruction {
            Operation::Cpy(x, y) => {
                let value = self.get_immediate_or_register_value(x);
                *self.registers.entry(y.to_string()).or_default() = value;
            }
            Operation::Inc(x) => *self.registers.entry(x.to_string()).or_default() += 1,
            Operation::Dec(x) => *self.registers.entry(x.to_string()).or_default() -= 1,
            Operation::Jnz(x, y) => {
                if self.get_immediate_or_register_value(x) != 0 {
                    return *y;
                }
            }
        }
        1
    }

    fn get_immediate_or_register_value(&self, x: &str) -> i32 {
        match x.parse::<i32>() {
            Ok(val) => val,
            Err(_) => *self.registers.get(x).unwrap_or(&0),
        }
    }
}

#[derive(Clone)]
enum Operation {
    Cpy(String, String),
    Inc(String),
    Dec(String),
    Jnz(String, i32),
}

impl Operation {
    fn from(line: &str) -> Self {
        let parts = line.split_whitespace().collect_vec();
        match parts[0] {
            "cpy" => Operation::Cpy(parts[1].to_string(), parts[2].to_string()),
            "inc" => Operation::Inc(parts[1].to_string()),
            "dec" => Operation::Dec(parts[1].to_string()),
            "jnz" => Operation::Jnz(parts[1].to_string(), parts[2].parse().unwrap()),
            _ => unreachable!(),
        }
    }
}

pub fn part1(input: &str) -> i32 {
    let instructions = parse(input);
    let mut computer = Computer::new(instructions);
    computer.run();

    *computer.registers.get("a").unwrap()
}

pub fn part2(input: &str) -> i32 {
    let instructions = parse(input);
    let mut computer = Computer::new(instructions);
    computer.registers.insert("c".to_string(), 1);
    computer.run();

    *computer.registers.get("a").unwrap()
}

fn parse(input: &str) -> Vec<Operation> {
    input.lines().map(Operation::from).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../examples/day12.txt");

    #[test]
    fn offset_calculated_correctly() {
        let mut computer = Computer::new(vec![]);
        let instructions = vec![
            Operation::Inc("a".to_string()),
            Operation::Jnz("a".to_string(), 10),
            Operation::Jnz("b".to_string(), -10),
            Operation::Jnz("a".to_string(), -10),
        ];

        assert_eq!(1, computer.execute_instruction(&instructions[0]));
        assert_eq!(10, computer.execute_instruction(&instructions[1]));
        assert_eq!(1, computer.execute_instruction(&instructions[2]));
        assert_eq!(-10, computer.execute_instruction(&instructions[3]));
    }

    #[test]
    fn registers_manipulated_correctly() {
        let mut computer = Computer::new(vec![]);
        let instructions = vec![
            Operation::Inc("a".to_string()),
            Operation::Dec("b".to_string()),
            Operation::Cpy("10".to_string(), "c".to_string()),
            Operation::Cpy("b".to_string(), "d".to_string()),
        ];

        instructions.iter().for_each(|instr| {
            computer.execute_instruction(instr);
        });

        assert_eq!(1, *computer.registers.get("a").unwrap());
        assert_eq!(-1, *computer.registers.get("b").unwrap());
        assert_eq!(10, *computer.registers.get("c").unwrap());
        assert_eq!(-1, *computer.registers.get("d").unwrap());
    }

    #[test]
    fn part1_ex() {
        assert_eq!(42, part1(INPUT));
    }
}
