use std::{cell::RefCell, collections::VecDeque};

struct Monkey<'a> {
    items: RefCell<VecDeque<u64>>,
    operation: (&'a str, &'a str),
    divisor: u64,
    throw_true: usize,
    throw_false: usize,
    throw_count: RefCell<usize>,
    worry_relief: Box<dyn Fn(u64) -> u64>,
}

impl<'a> Monkey<'a> {
    fn new(
        items: VecDeque<u64>,
        operation: (&'a str, &'a str),
        divisor: u64,
        throw_true: usize,
        throw_false: usize,
        worry_relief: fn(u64) -> u64,
    ) -> Monkey<'a> {
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
            ("*", "old") => value * value,
            ("+", "old") => value + value,
            ("*", num) => value * num.parse::<u64>().unwrap(),
            ("+", num) => value + num.parse::<u64>().unwrap(),
            _ => unreachable!(),
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

    println!("{divisor_prod}");
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

    const INPUT: &str = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn part1_ex() {
        assert_eq!(10605, part1(INPUT));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(2713310158, part2(INPUT));
    }
}
