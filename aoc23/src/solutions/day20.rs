use std::collections::{HashMap, VecDeque};

use num::Integer;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Signal {
    High,
    Low,
}

struct Module {
    outputs: Vec<String>,
    behavior: Box<dyn ReceiveSignal>,
}

trait ReceiveSignal {
    fn receive_signal(&mut self, source: &str, incoming: Signal) -> Option<Signal>;
}

struct Broadcast;

impl ReceiveSignal for Broadcast {
    fn receive_signal(&mut self, _: &str, incoming: Signal) -> Option<Signal> {
        Some(incoming)
    }
}

#[derive(Default)]
struct FlipFlop {
    on: bool,
}

impl ReceiveSignal for FlipFlop {
    fn receive_signal(&mut self, _: &str, incoming: Signal) -> Option<Signal> {
        match incoming {
            Signal::High => None,
            Signal::Low => {
                self.on = !self.on;
                Some(if self.on { Signal::High } else { Signal::Low })
            }
        }
    }
}

#[derive(Default)]
struct Conjunction {
    memory: HashMap<String, Signal>,
}

impl ReceiveSignal for Conjunction {
    fn receive_signal(&mut self, source: &str, incoming: Signal) -> Option<Signal> {
        self.memory.insert(source.to_string(), incoming);

        Some(
            if self.memory.values().all(|signal| signal == &Signal::High) {
                Signal::Low
            } else {
                Signal::High
            },
        )
    }
}

pub fn part1(input: &str) -> usize {
    let mut modules = parse(input);

    let (high_count, low_count) = (0..1_000)
        .map(|_| simulate(&mut modules))
        .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));

    high_count * low_count
}

pub fn part2(input: &str) -> usize {
    // Solution specifically tailored for my input. 
    // Identified the "parents" of the rx node and find how many iterations are needed until they are all High
    // Calculate LCM for the resulting values to find the first time when all of them are true
    ["xm", "dr", "nh", "tr"]
        .iter()
        .map(|target| {
            let mut modules = parse(input);

            simulate_until_high(&mut modules, target)
        })
        .fold(1, |x, y| x.lcm(&y))
}

fn simulate(modules: &mut HashMap<String, Module>) -> (usize, usize) {
    let mut sent_signals =
        VecDeque::from([("button".to_string(), "broadcaster".to_string(), Signal::Low)]);
    let mut low_count = 0;
    let mut high_count = 0;

    while let Some((source, destination, signal)) = sent_signals.pop_front() {
        match signal {
            Signal::Low => {
                low_count += 1;
            }
            Signal::High => {
                high_count += 1;
            }
        }

        if let Some(values) = evaluate_signal(modules, source, destination, signal) {
            sent_signals.extend(values.into_iter());
        }
    }

    (low_count, high_count)
}

fn evaluate_signal(
    modules: &mut HashMap<String, Module>,
    source: String,
    destination: String,
    signal: Signal,
) -> Option<Vec<(String, String, Signal)>> {
    if let Some(module) = modules.get_mut(&destination) {
        if let Some(next_signal) = module.behavior.receive_signal(&source, signal) {
            return Some(
                module
                    .outputs
                    .iter()
                    .map(|output| (destination.clone(), output.clone(), next_signal))
                    .collect(),
            );
        }
    }
    None
}

fn simulate_until_high(modules: &mut HashMap<String, Module>, target: &str) -> usize {
    let mut presses = 0;
    loop {
        presses += 1;

        let mut sent_signals =
            VecDeque::from([("button".to_string(), "broadcaster".to_string(), Signal::Low)]);

        while let Some((source, destination, signal)) = sent_signals.pop_front() {
            if source == target && signal == Signal::High {
                return presses;
            }
            if let Some(values) = evaluate_signal(modules, source, destination, signal) {
                sent_signals.extend(values.into_iter());
            }
        }
    }
}

fn parse(input: &str) -> HashMap<String, Module> {
    let mut module_map: HashMap<String, Module> = HashMap::new();

    let mut outputs_of: HashMap<String, Vec<String>> = HashMap::new();

    // build map which module is output of which modules. This is important for Conjunction modules
    for line in input.lines() {
        let (name, outputs) = line.split_once(" -> ").unwrap();

        for output in outputs.split(", ") {
            outputs_of
                .entry(output.to_string())
                .or_default()
                .push(process_name(name));
        }
    }

    // build map of modules
    for line in input.lines() {
        let (name, outputs) = line.split_once(" -> ").unwrap();
        let outputs: Vec<String> = outputs.split(", ").map(|out| out.to_string()).collect();

        let (name, behavior): (String, Box<dyn ReceiveSignal>) = if name.starts_with('%') {
            (process_name(name), Box::<FlipFlop>::default())
        } else if name.starts_with('&') {
            let name = process_name(name);

            let memory = outputs_of
                .get(&name)
                .unwrap()
                .iter()
                .map(|output| (output.to_string(), Signal::Low))
                .collect();

            (name, Box::new(Conjunction { memory }))
        } else {
            (name.to_string(), Box::new(Broadcast))
        };

        module_map.insert(name, Module { outputs, behavior });
    }

    module_map
}

fn process_name(raw_name: &str) -> String {
    if raw_name.starts_with('%') || raw_name.starts_with('&') {
        raw_name.chars().skip(1).collect()
    } else {
        raw_name.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = include_str!("../../examples/day20_1.txt");
    const EXAMPLE_2: &str = include_str!("../../examples/day20_2.txt");

    #[test]
    fn part1_ex() {
        assert_eq!(32_000_000, part1(EXAMPLE_1));
        assert_eq!(11_687_500, part1(EXAMPLE_2));
    }
}
