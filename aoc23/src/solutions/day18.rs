use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use util::grid::Direction;

lazy_static! {
    static ref DIG_INSTRUCTION_REGEX: Regex =
        Regex::new(r"^([RULD]) (\d+) \(#([0-9a-f]{6})\)$").unwrap();
}

#[derive(Debug, Default, Clone, Copy)]
struct Coordinate {
    pub x: i64,
    pub y: i64,
}

impl Coordinate {
    pub fn from(x: i64, y: i64) -> Self {
        Coordinate { x, y }
    }

    fn step(&self, direction: &Direction, steps: u32) -> Self {
        let steps = steps as i64;
        match direction {
            Direction::Right => Coordinate::from(self.x + steps, self.y),
            Direction::Left => Coordinate::from(self.x - steps, self.y),
            Direction::Up => Coordinate::from(self.x, self.y - steps),
            Direction::Down => Coordinate::from(self.x, self.y + steps),
        }
    }
}

struct Instruction(Direction, u32);

pub fn part1(input: &str) -> usize {
    let instructions = parse(input, parse_instruction);
    let (edges, boundary_count) = dig_pit(&instructions);
    total_area(&edges, boundary_count)
}

pub fn part2(input: &str) -> usize {
    let instructions = parse(input, parse_extracted_instruction);
    let (edges, boundary_count) = dig_pit(&instructions);
    total_area(&edges, boundary_count)
}

fn dig_pit(instructions: &[Instruction]) -> (Vec<Coordinate>, usize) {
    let mut edges = Vec::with_capacity(instructions.len());
    let mut boundary_count = 0;
    let mut current = Coordinate::default();

    for Instruction(direction, steps) in instructions {
        current = current.step(direction, *steps);
        edges.push(current);

        boundary_count += steps;
    }

    (edges, boundary_count as usize)
}

fn total_area(pit: &[Coordinate], boundary_count: usize) -> usize {
    let area = calculate_area(pit);
    boundary_count + interior_count(area, boundary_count)
}

fn calculate_area(polygon: &[Coordinate]) -> usize {
    // Calculate the area of the polygon with the Shoelace formula (https://en.wikipedia.org/wiki/Shoelace_formula).

    let first_edge = [polygon[0]];
    let polygon = polygon.into_iter().chain(first_edge.iter()).collect_vec();

    let area: i64 = polygon
        .iter()
        .zip(polygon.iter().skip(1))
        .map(|(p1, p2)| (p1.x * p2.y) - (p1.y * p2.x))
        .sum();

    area.unsigned_abs() as usize / 2
}

fn interior_count(area: usize, boundary_count: usize) -> usize {
    // Calculate the number of points inside the polygon with the Pick's Theorem (https://en.wikipedia.org/wiki/Pick%27s_theorem)
    area - boundary_count / 2 + 1
}

fn parse(input: &str, parse_instruction_function: fn(&str) -> Instruction) -> Vec<Instruction> {
    input.lines().map(parse_instruction_function).collect()
}

fn parse_instruction(line: &str) -> Instruction {
    let captures = DIG_INSTRUCTION_REGEX.captures_iter(line).next().unwrap();

    Instruction(
        Direction::from(captures[1].chars().next().unwrap()),
        captures[2].parse().unwrap(),
    )
}

fn parse_extracted_instruction(line: &str) -> Instruction {
    let captures = DIG_INSTRUCTION_REGEX.captures_iter(line).next().unwrap();
    let hex_code = captures[3].to_string();

    let steps =
        u32::from_str_radix(hex_code.chars().take(5).collect::<String>().as_str(), 16).unwrap();
    let direction = match hex_code.chars().skip(5).next().unwrap() {
        '0' => Direction::Right,
        '1' => Direction::Down,
        '2' => Direction::Left,
        '3' => Direction::Up,
        _ => unreachable!(),
    };

    Instruction(direction, steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../examples/day18.txt");

    #[test]
    fn part1_ex() {
        assert_eq!(62, part1(EXAMPLE));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(952408144115, part2(EXAMPLE));
    }
}
