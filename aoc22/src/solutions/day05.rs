use regex::Regex;
use std::collections::VecDeque;

struct CargoCrane {
    stacks: Vec<Vec<char>>,
}

impl CargoCrane {
    fn move_crates(&mut self, num: usize, from: usize, to: usize) {
        for _ in 0..num {
            let char = self.stacks[from].pop().unwrap();
            self.stacks[to].push(char);
        }
    }

    fn move_crates_simultaniously(&mut self, num: usize, from: usize, to: usize) {
        let mut picked_up: VecDeque<char> = VecDeque::with_capacity(num);
        for _ in 0..num {
            picked_up.push_front(self.stacks[from].pop().unwrap());
        }

        for char in picked_up {
            self.stacks[to].push(char);
        }
    }

    fn view_top(&self) -> String {
        self.stacks
            .iter()
            .map(|stack| stack.last().unwrap())
            .collect()
    }
}

pub fn part1(input: &str) -> String {
    let (mut crane, instructions) = parse(input);

    for (num, from, to) in instructions {
        crane.move_crates(num, from, to);
    }

    crane.view_top()
}

pub fn part2(input: &str) -> String {
    let (mut crane, instructions) = parse(input);

    for (num, from, to) in instructions {
        crane.move_crates_simultaniously(num, from, to);
    }

    crane.view_top()
}

fn parse(input: &str) -> (CargoCrane, Vec<(usize, usize, usize)>) {
    let (stack_input, instruction_input) = input.split_once("\n\n").unwrap();

    (
        parse_stacks(stack_input),
        parse_instructions(instruction_input),
    )
}

fn parse_stacks(stack_input: &str) -> CargoCrane {
    let mut stack_input = stack_input.lines().rev();
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); (stack_input.next().unwrap().len() + 1) / 4];

    for row in stack_input {
        let chars: Vec<char> = row.chars().collect();
        for i in 0..stacks.len() {
            match chars[1 + i * 4] {
                c @ 'A'..='Z' => stacks[i].push(c),
                ' ' => {} // ignore this
                _ => unreachable!(),
            }
        }
    }

    CargoCrane { stacks }
}

fn parse_instructions(instruction_input: &str) -> Vec<(usize, usize, usize)> {
    let move_regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    instruction_input
        .lines()
        .map(|line| {
            let capture = move_regex.captures_iter(line).next().unwrap();
            let numbers: Vec<usize> = (1..=3).map(|i| capture[i].parse().unwrap()).collect();

            (numbers[0], numbers[1] - 1, numbers[2] - 1) // -1 because of array indices
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part1_ex() {
        assert_eq!("CMZ".to_string(), part1(INPUT));
    }

    #[test]
    fn part2_ex() {
        assert_eq!("MCD".to_string(), part2(INPUT));
    }
}
