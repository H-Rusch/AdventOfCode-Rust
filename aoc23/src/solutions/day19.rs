use lazy_static::lazy_static;
use regex::{Match, Regex};
use std::collections::HashMap;

lazy_static! {
    static ref OPERATION_REGEX: Regex = Regex::new(r"^([xmas])[<>](\d+):(.*)$").unwrap();
}

enum Rule {
    GT(char, usize, Operation),
    LT(char, usize, Operation),
    Unconditional(Operation),
}

impl Rule {
    fn match_part(&self, part: &Part) -> Option<Operation> {
        match self {
            Rule::Unconditional(op) => Some(op.clone()),
            Rule::GT(field, val, op) => {
                if part.get_value(field) > *val {
                    Some(op.clone())
                } else {
                    None
                }
            }
            Rule::LT(field, val, op) => {
                if part.get_value(field) < *val {
                    Some(op.clone())
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Clone)]
enum Operation {
    Accept,
    Reject,
    Call(String),
}

struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn get_value(&self, field: &char) -> usize {
        match field {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => unreachable!(),
        }
    }

    fn sum_ratings(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

pub fn part1(input: &str) -> usize {
    let (workflows, parts) = parse(input);

    parts
        .iter()
        .filter(|part| is_part_accepted(part, &workflows, "in"))
        .map(|part| part.sum_ratings())
        .sum()
}

pub fn part2(_input: &str) -> usize {
    0
}

fn is_part_accepted(part: &Part, workflows: &HashMap<String, Vec<Rule>>, workflow: &str) -> bool {
    let operation = workflows
        .get(workflow)
        .unwrap()
        .iter()
        .find_map(|rule| rule.match_part(part))
        .expect("the last rule has to be unconditional");

    match operation {
        Operation::Accept => true,
        Operation::Reject => false,
        Operation::Call(other) => is_part_accepted(part, workflows, &other),
    }
}

fn parse(input: &str) -> (HashMap<String, Vec<Rule>>, Vec<Part>) {
    let (workflow_part, rating_part) = input.split_once("\n\n").unwrap();

    let workflows = parse_workflows(workflow_part);
    let ratings = parse_parts(rating_part);

    (workflows, ratings)
}

fn parse_workflows(workflow_part: &str) -> HashMap<String, Vec<Rule>> {
    let workflow_regex = Regex::new(r"(.+)\{(.*)\}").unwrap();
    workflow_part
        .lines()
        .map(|line| {
            let captures = workflow_regex.captures(line).unwrap();

            let name = captures.get(1).unwrap().as_str().to_string();
            let rules = captures
                .get(2)
                .unwrap()
                .as_str()
                .split(',')
                .map(parse_rule)
                .collect();

            (name, rules)
        })
        .collect()
}

fn parse_rule(rule_input: &str) -> Rule {
    if rule_input.contains('<') {
        let captures = OPERATION_REGEX.captures(rule_input).unwrap();

        Rule::LT(
            captures.get(1).unwrap().as_str().chars().next().unwrap(),
            parse_usize(captures.get(2)),
            parse_operation(captures.get(3).unwrap().as_str()),
        )
    } else if rule_input.contains('>') {
        let captures = OPERATION_REGEX.captures(rule_input).unwrap();

        Rule::GT(
            captures.get(1).unwrap().as_str().chars().next().unwrap(),
            parse_usize(captures.get(2)),
            parse_operation(captures.get(3).unwrap().as_str()),
        )
    } else {
        Rule::Unconditional(parse_operation(rule_input))
    }
}

fn parse_operation(operation_part: &str) -> Operation {
    match operation_part {
        "A" => Operation::Accept,
        "R" => Operation::Reject,
        other => Operation::Call(other.to_string()),
    }
}

fn parse_parts(parts_part: &str) -> Vec<Part> {
    let part_regex = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();

    parts_part
        .lines()
        .map(|line| {
            let captures = part_regex.captures(line).unwrap();

            Part {
                x: parse_usize(captures.get(1)),
                m: parse_usize(captures.get(2)),
                a: parse_usize(captures.get(3)),
                s: parse_usize(captures.get(4)),
            }
        })
        .collect()
}

fn parse_usize(capture: Option<Match>) -> usize {
    capture.unwrap().as_str().parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../examples/day19.txt");

    #[test]
    fn part1_ex() {
        assert_eq!(19114, part1(EXAMPLE));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(0, part2(EXAMPLE));
    }
}
