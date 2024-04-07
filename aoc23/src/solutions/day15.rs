use lazy_static::lazy_static;
use regex::Regex;
use std::collections::VecDeque;

lazy_static! {
    static ref STEP_REGEX: Regex = Regex::new(r"^(\w*)([-=])(\w?)$").unwrap();
}

const LIMIT: usize = 256;

type Lense = (String, usize);
enum Operation {
    Write,
    Remove,
}

pub fn part1(input: &str) -> usize {
    input.split(',').map(calculate_hash).sum()
}

pub fn part2(input: &str) -> usize {
    let mut lense_boxes: Vec<VecDeque<Lense>> = (0..LIMIT).map(|_| VecDeque::new()).collect();

    for step in input.split(',') {
        let (label, operation, focal_length) = parse_step(step);
        let lense_box = &mut lense_boxes[calculate_hash(&label)];

        match operation {
            Operation::Write => {
                if let Some(index) = lense_box.iter().position(|lense| lense.0 == label) {
                    lense_box[index] = (label, focal_length);
                } else {
                    lense_box.push_back((label, focal_length));
                }
            }
            Operation::Remove => {
                if let Some(index) = lense_box.iter().position(|lense| lense.0 == label) {
                    lense_box.remove(index);
                }
            }
        }
    }

    calculate_focusing_power(&lense_boxes)
}

fn calculate_hash(to_hash: &str) -> usize {
    to_hash.chars().fold(0, |acc, ch| ((acc + ch as usize) * 17) % LIMIT)
}

fn parse_step(step: &str) -> (String, Operation, usize) {
    let captures = STEP_REGEX.captures_iter(step).next().unwrap();
    (
        captures[1].to_string(),
        match &captures[2] {
            "=" => Operation::Write,
            "-" => Operation::Remove,
            _ => unreachable!(),
        },
        captures[3].parse().unwrap_or_default(),
    )
}

fn calculate_focusing_power(lense_boxes: &[VecDeque<Lense>]) -> usize {
    lense_boxes
        .iter()
        .enumerate()
        .flat_map(|(box_index, lense_box)| {
            lense_box
                .iter()
                .enumerate()
                .map(|(lense_index, lense)| (1 + box_index) * (lense_index + 1) * lense.1)
                .collect::<Vec<usize>>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_hash() {
        assert_eq!(52, calculate_hash("HASH"));
        assert_eq!(30, calculate_hash("rn=1"));
        assert_eq!(253, calculate_hash("cm-"));
        assert_eq!(97, calculate_hash("qp=3"));
        assert_eq!(47, calculate_hash("cm=2"));
        assert_eq!(14, calculate_hash("qp-"));
        assert_eq!(180, calculate_hash("pc=4"));
    }

    #[test]
    fn test_calculating_focusing_power_for_lense_box() {
        let lense_boxes = vec![
            VecDeque::from(vec![("rn".to_string(), 1), ("cm".to_string(), 2)]),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::from(vec![
                ("ot".to_string(), 7),
                ("ab".to_string(), 5),
                ("pc".to_string(), 6),
            ]),
        ];

        assert_eq!(145, calculate_focusing_power(&lense_boxes));
    }

    #[test]
    fn part1_ex() {
        assert_eq!(1320, part1(EXAMPLE));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(145, part2(EXAMPLE));
    }
}
