use lazy_static::lazy_static;
use regex::{Match, Regex};
use std::{collections::HashMap, ops::Range};

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

#[derive(Clone, Debug)]
struct PartRanges {
    x: Range<usize>,
    m: Range<usize>,
    a: Range<usize>,
    s: Range<usize>,
}

impl PartRanges {
    const LIMIT: usize = 4001;

    fn default() -> Self {
        PartRanges {
            x: 1..PartRanges::LIMIT,
            m: 1..PartRanges::LIMIT,
            a: 1..PartRanges::LIMIT,
            s: 1..PartRanges::LIMIT,
        }
    }

    fn split(&self, field: char, value: usize) -> (Self, Self) {
        let mut lower = self.clone();
        let mut higher = self.clone();

        match field {
            'x' => {
                lower.x = lower.x.start..value;
                higher.x = value..higher.x.end;
            }
            'm' => {
                lower.m = lower.m.start..value;
                higher.m = value..higher.m.end;
            }
            'a' => {
                lower.a = lower.a.start..value;
                higher.a = value..higher.a.end;
            }
            's' => {
                lower.s = lower.s.start..value;
                higher.s = value..higher.s.end;
            }
            _ => unreachable!(),
        }

        (lower, higher)
    }

    fn score(&self) -> usize {
        self.x.len() * self.m.len() * self.a.len() * self.s.len()
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

pub fn part2(input: &str) -> usize {
    let (workflows, _) = parse(input);

    let mut accepted = Vec::new();
    get_valid_ranges(PartRanges::default(), &workflows, "in", &mut accepted);

    accepted.iter().map(PartRanges::score).sum()
}

fn get_valid_ranges(
    range: PartRanges,
    workflows: &HashMap<String, Vec<Rule>>,
    workflow: &str,
    accepted: &mut Vec<PartRanges>,
) {
    let mut current = range;
    for rule in workflows.get(workflow).unwrap() {
        match rule {
            Rule::GT(field, value, op) => {
                let (disallowed, allowed) = current.split(*field, value + 1);
                current = disallowed;

                evaluate_op(allowed, workflows, op, accepted);
            }
            Rule::LT(field, value, op) => {
                let (allowed, disallowed) = current.split(*field, *value);
                current = disallowed;

                evaluate_op(allowed, workflows, op, accepted);
            }
            Rule::Unconditional(op) => {
                evaluate_op(current.clone(), workflows, op, accepted);
            }
        }
    }
}

fn evaluate_op(
    range: PartRanges,
    workflows: &HashMap<String, Vec<Rule>>,
    op: &Operation,
    accepted: &mut Vec<PartRanges>,
) {
    match op {
        Operation::Reject => (),
        Operation::Accept => accepted.push(range),
        Operation::Call(other) => get_valid_ranges(range, workflows, other, accepted),
    }
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
        assert_eq!(167_409_079_868_000, part2(EXAMPLE));
    }
}
