use itertools::Itertools;
use regex::Regex;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

struct Bot {
    id: u32,
    microchips: Vec<u32>,
    lower_to: Option<Rc<RefCell<dyn Give>>>,
    higher_to: Option<Rc<RefCell<dyn Give>>>,
}

impl Bot {
    fn new(id: u32) -> Self {
        Bot {
            id,
            microchips: Vec::new(),
            lower_to: None,
            higher_to: None,
        }
    }

    fn evaluate(&mut self) {
        if self.microchips.len() != 2 {
            return;
        }

        let (higher, lower) = (
            self.microchips.pop().unwrap(),
            self.microchips.pop().unwrap(),
        );
        self.lower_to
            .as_mut()
            .unwrap()
            .borrow_mut()
            .give_value(lower);
        self.higher_to
            .as_mut()
            .unwrap()
            .borrow_mut()
            .give_value(higher);
    }

    fn test_for_value(&self, value: (u32, u32)) -> bool {
        if self.microchips.len() != 2 {
            return false;
        }

        value == (self.microchips[0], self.microchips[1])
    }
}

struct Output {
    _id: u32,
    microchips: Vec<u32>,
}

impl Output {
    fn new(id: u32) -> Self {
        Output {
            _id: id,
            microchips: Vec::new(),
        }
    }
}

trait Give {
    fn give_value(&mut self, value: u32);
}

impl Give for Bot {
    fn give_value(&mut self, chip: u32) {
        self.microchips.push(chip);
        self.microchips.sort();
    }
}

impl Give for Output {
    fn give_value(&mut self, chip: u32) {
        self.microchips.push(chip);
    }
}

pub fn part1(input: &str) -> u32 {
    let test_for = (17, 61);
    let (bots, _) = parse(input);

    loop {
        for bot in bots.iter() {
            if bot.borrow().test_for_value(test_for) {
                return bot.borrow().id;
            }

            bot.borrow_mut().evaluate();
        }
    }
}

pub fn part2(input: &str) -> u32 {
    let (bots, outputs) = parse(input);

    loop {
        for bot in bots.iter() {
            bot.borrow_mut().evaluate();
        }
        if (0..3).all(|i| !outputs.get(&i).unwrap().borrow().microchips.is_empty()) {
            return (0..3)
                .map(|i| outputs.get(&i).unwrap().borrow().microchips[0])
                .product();
        }
    }
}

type OutputMap = HashMap<u32, Rc<RefCell<Output>>>;
type BotVec = Vec<Rc<RefCell<Bot>>>;

fn parse(input: &str) -> (BotVec, OutputMap) {
    let mut bots = HashMap::new();
    let mut outputs = HashMap::new();

    create_initial_bots(&mut bots, input);
    create_outputs(&mut outputs, input);
    connect_and_fill_bots(&mut bots, &mut outputs, input);

    (bots.into_values().collect(), outputs)
}

fn create_initial_bots(bots: &mut HashMap<u32, Rc<RefCell<Bot>>>, input: &str) {
    let bot_pattern = Regex::new(r"bot (\d+)").unwrap();
    for capture in bot_pattern.captures_iter(input) {
        let id = capture.get(1).unwrap().as_str().parse().unwrap();
        bots.insert(id, Rc::new(RefCell::new(Bot::new(id))));
    }
}

fn create_outputs(outputs: &mut OutputMap, input: &str) {
    let output_pattern = Regex::new(r"output (\d+)").unwrap();
    for capture in output_pattern.captures_iter(input) {
        let id = capture.get(1).unwrap().as_str().parse().unwrap();
        outputs.insert(id, Rc::new(RefCell::new(Output::new(id))));
    }
}

fn connect_and_fill_bots(
    bots: &mut HashMap<u32, Rc<RefCell<Bot>>>,
    outputs: &mut OutputMap,
    input: &str,
) {
    let give_pattern =
        Regex::new(r"bot (\d+) gives low to (output|bot) (\d+) and high to (output|bot) (\d+)")
            .unwrap();
    for line in input.lines() {
        if line.starts_with("value") {
            let parts = line.split_whitespace().collect_vec();
            let value = parts[1].parse().unwrap();
            let id = parts[5].parse::<u32>().unwrap();

            bots.get_mut(&id).unwrap().borrow_mut().give_value(value);
        } else {
            let captures = give_pattern.captures(line).unwrap();
            let bot_id = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();

            // bind lower_value
            let connect_to = get_object_to_bind(
                bots,
                outputs,
                captures.get(2).unwrap().as_str(),
                captures.get(3).unwrap().as_str().parse::<u32>().unwrap(),
            );
            bots.get(&bot_id).unwrap().borrow_mut().lower_to = Some(connect_to);

            // bind higer_value
            let connect_to = get_object_to_bind(
                bots,
                outputs,
                captures.get(4).unwrap().as_str(),
                captures.get(5).unwrap().as_str().parse::<u32>().unwrap(),
            );
            bots.get(&bot_id).unwrap().borrow_mut().higher_to = Some(connect_to);
        }
    }
}

fn get_object_to_bind(
    bots: &mut HashMap<u32, Rc<RefCell<Bot>>>,
    outputs: &mut OutputMap,
    kind: &str,
    id: u32,
) -> Rc<RefCell<dyn Give>> {
    match kind {
        "bot" => bots.get(&id).unwrap().clone(),
        "output" => outputs.get(&id).unwrap().clone(),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../examples/day10.txt");

    fn get_by_id(bots: &Vec<Rc<RefCell<Bot>>>, id: u32) -> &RefCell<Bot> {
        bots.iter().find(|b| b.borrow().id == id).unwrap()
    }

    #[test]
    fn components_created_successfully() {
        let (bots, outputs) = parse(INPUT);

        assert_eq!(3, bots.len());
        assert_eq!(3, outputs.values().len());
        assert_eq!(0, get_by_id(&bots, 0).borrow().microchips.len());
        assert_eq!(1, get_by_id(&bots, 1).borrow().microchips.len());
        assert!(get_by_id(&bots, 1).borrow().microchips.contains(&3));
        assert_eq!(2, get_by_id(&bots, 2).borrow().microchips.len());
        assert!(get_by_id(&bots, 2).borrow().microchips.contains(&2));
        assert!(get_by_id(&bots, 2).borrow().microchips.contains(&5));
    }

    #[test]
    fn handing_off_values_works_successfully() {
        let (bots, _) = parse(INPUT);

        get_by_id(&bots, 2).borrow_mut().evaluate();

        assert_eq!(1, get_by_id(&bots, 0).borrow().microchips.len());
        assert!(get_by_id(&bots, 0).borrow().microchips.contains(&5));
        assert_eq!(2, get_by_id(&bots, 1).borrow().microchips.len());
        assert!(get_by_id(&bots, 1).borrow().microchips.contains(&2));
        assert!(get_by_id(&bots, 1).borrow().microchips.contains(&3));
        assert_eq!(0, get_by_id(&bots, 2).borrow().microchips.len());
    }

    #[test]
    fn handing_off_full_example_arrives_at_correct_end_state() {
        let (bots, outputs) = parse(INPUT);

        get_by_id(&bots, 2).borrow_mut().evaluate();
        get_by_id(&bots, 1).borrow_mut().evaluate();
        get_by_id(&bots, 0).borrow_mut().evaluate();

        assert_eq!(0, get_by_id(&bots, 0).borrow().microchips.len());
        assert_eq!(0, get_by_id(&bots, 1).borrow().microchips.len());
        assert_eq!(0, get_by_id(&bots, 2).borrow().microchips.len());
        assert!(outputs.get(&0).unwrap().borrow().microchips.contains(&5));
        assert!(outputs.get(&1).unwrap().borrow().microchips.contains(&2));
        assert!(outputs.get(&2).unwrap().borrow().microchips.contains(&3));
    }
}
