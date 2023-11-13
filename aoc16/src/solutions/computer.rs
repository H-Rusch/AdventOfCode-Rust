use std::collections::HashMap;

use itertools::Itertools;

pub struct Computer {
    instructions: Vec<Operation>,
    registers: HashMap<String, i32>,
    ic: usize,
}

impl Computer {
    pub fn new(instructions: Vec<Operation>) -> Self {
        Computer {
            instructions,
            registers: HashMap::new(),
            ic: 0,
        }
    }

    pub fn get(&self, key: &str) -> Option<i32> {
        self.registers.get(key).copied()
    }

    pub fn insert(&mut self, key: String, value: i32) {
        self.registers.insert(key, value);
    }

    pub fn run(&mut self) {
        while let Some(instruction) = self.instructions.get(self.ic) {
            let offset = self.execute_instruction(&instruction.clone());
            if offset < 0 {
                self.ic -= offset.unsigned_abs() as usize;
            } else {
                self.ic += offset as usize;
            }
        }
    }

    pub fn execute_instruction(&mut self, instruction: &Operation) -> i32 {
        match instruction {
            Operation::Cpy(x, y) => {
                let value = self.get_immediate_or_register_value(x);
                if self.is_valid_register(y) {
                    *self.registers.entry(y.to_string()).or_default() = value;
                }
            }
            Operation::Inc(x) => *self.registers.entry(x.to_string()).or_default() += 1,
            Operation::Dec(x) => *self.registers.entry(x.to_string()).or_default() -= 1,
            Operation::Jnz(x, y) => {
                // skip invalid instruction
                if self.get_immediate_or_register_value(x) != 0 {
                    return self.get_immediate_or_register_value(y);
                }
            }
            Operation::Tgl(x) => {
                let offset = self.get_immediate_or_register_value(x);
                if let Some(instr) = self.instructions.get((offset + self.ic as i32) as usize) {
                    let toggled = self.toggle(instr);
                    self.instructions[(offset + self.ic as i32) as usize] = toggled;
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

    fn is_valid_register(&self, identifier: &str) -> bool {
        identifier.parse::<i32>().is_err()
    }

    fn toggle(&self, operation: &Operation) -> Operation {
        match operation {
            Operation::Inc(x) => Operation::Dec(x.clone()),
            Operation::Dec(x) | Operation::Tgl(x) => Operation::Inc(x.clone()),
            Operation::Jnz(x, y) => Operation::Cpy(x.clone(), y.to_string()),
            Operation::Cpy(x, y) => Operation::Jnz(x.clone(), y.clone()),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Operation {
    Cpy(String, String),
    Inc(String),
    Dec(String),
    Jnz(String, String),
    Tgl(String),
}

impl Operation {
    pub fn from(line: &str) -> Self {
        let parts = line.split_whitespace().collect_vec();
        match parts[0] {
            "cpy" => Operation::Cpy(parts[1].to_string(), parts[2].to_string()),
            "inc" => Operation::Inc(parts[1].to_string()),
            "dec" => Operation::Dec(parts[1].to_string()),
            "jnz" => Operation::Jnz(parts[1].to_string(), parts[2].parse().unwrap()),
            "tgl" => Operation::Tgl(parts[1].to_string()),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offset_calculated_correctly() {
        let mut computer = Computer::new(vec![]);
        let instructions = vec![
            Operation::Inc("a".to_string()),
            Operation::Jnz("a".to_string(), "10".to_string()),
            Operation::Jnz("b".to_string(), "-10".to_string()),
            Operation::Jnz("a".to_string(), "-10".to_string()),
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

        assert_eq!(1, computer.get("a").unwrap());
        assert_eq!(-1, computer.get("b").unwrap());
        assert_eq!(10, computer.get("c").unwrap());
        assert_eq!(-1, computer.get("d").unwrap());
    }
}
