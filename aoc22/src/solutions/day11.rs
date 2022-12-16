use std::{cell::RefCell, collections::VecDeque};

enum Op {
    Square,
    Add(u64),
    Mult(u64),
}

struct Monkey {
    items: RefCell<VecDeque<u64>>,
    operation: Op,
    divisor: u64,
    throw_true: usize,
    throw_false: usize,
    throw_count: RefCell<usize>,
    worry_relief: Box<dyn Fn(u64) -> u64>,
}

impl Monkey {
    fn new(
        items: VecDeque<u64>,
        operation: Op,
        divisor: u64,
        throw_true: usize,
        throw_false: usize,
        worry_relief: fn(u64) -> u64,
    ) -> Monkey {
        Monkey {
            items: RefCell::new(items),
            operation,
            divisor,
            throw_true,
            throw_false,
            throw_count: RefCell::new(0),
            worry_relief: Box::new(worry_relief),
        }
    }

    fn execute_turn(&self, monkeys: &[Monkey]) {
        *self.throw_count.borrow_mut() += self.items.borrow().len();

        while let Some(mut item) = self.items.borrow_mut().pop_front() {
            item = self.calc_worry_level(item);
            item = (self.worry_relief)(item);

            if item % self.divisor == 0 {
                monkeys[self.throw_true].items.borrow_mut().push_back(item);
            } else {
                monkeys[self.throw_false].items.borrow_mut().push_back(item);
            }
        }
    }

    fn calc_worry_level(&self, value: u64) -> u64 {
        match self.operation {
            Op::Square => value * value,
            Op::Add(n) => value + n,
            Op::Mult(n) => value * n, 
        }
    }
}

pub fn part1(input: &str) -> usize {
    let mut monkeys = parse(input);

    perform_iteration(20, &monkeys);

    calc_monkey_business(&mut monkeys)
}

pub fn part2(input: &str) -> usize {
    let mut monkeys = parse(input);
    // change worry relief function
    let divisor_prod: u64 = monkeys.iter().map(|monkey| monkey.divisor).product();
    for monkey in monkeys.iter_mut() {
        monkey.worry_relief = Box::new(move |x: u64| x % divisor_prod);
    }

    perform_iteration(10_000, &monkeys);

    calc_monkey_business(&mut monkeys)
}

fn perform_iteration(limit: usize, monkeys: &Vec<Monkey>) {
    for _ in 0..limit {
        for i in 0..monkeys.len() {
            monkeys[i].execute_turn(monkeys);
        }
    }
}

fn calc_monkey_business(monkeys: &mut [Monkey]) -> usize {
    monkeys.sort_by_key(|monkey| *monkey.throw_count.borrow());
    monkeys
        .iter()
        .rev()
        .take(2)
        .map(|monkey| *monkey.throw_count.borrow())
        .product()
}

fn parse(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|monkey| {
            let mut lines = monkey.lines();
            lines.next().unwrap();

            // starting items
            let items: VecDeque<u64> = lines
                .next()
                .unwrap()
                .strip_prefix("  Starting items: ")
                .unwrap()
                .split(", ")
                .map(|num| num.parse().unwrap())
                .collect();
            // Operation
            let operation = lines
                .next()
                .unwrap()
                .strip_prefix("  Operation: new = old ")
                .unwrap()
                .split_once(' ')
                .unwrap();
            let operation = match operation {
                    ("*", "old") => Op::Square,
                    //("+", "old") => value + value,
                    ("*", num) => Op::Mult(num.parse().unwrap()),
                    ("+", num) => Op::Add(num.parse().unwrap()),
                    _ => unreachable!(),
            };
            // divisor
            let divisor: u64 = lines
                .next()
                .unwrap()
                .strip_prefix("  Test: divisible by ")
                .unwrap()
                .parse()
                .unwrap();
            let throw_if_true: usize = lines
                .next()
                .unwrap()
                .strip_prefix("    If true: throw to monkey ")
                .unwrap()
                .parse()
                .unwrap();
            let throw_if_false: usize = lines
                .next()
                .unwrap()
                .strip_prefix("    If false: throw to monkey ")
                .unwrap()
                .parse()
                .unwrap();
            let worry_relief: fn(u64) -> u64 = |x| x / 3;

            Monkey::new(
                items,
                operation,
                divisor,
                throw_if_true,
                throw_if_false,
                worry_relief,
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../examples/day11.txt");

    #[test]
    fn part1_ex() {
        assert_eq!(10605, part1(INPUT));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(2713310158, part2(INPUT));
    }
}
