use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct EnginePart {
    value: usize,
    y: i32,
    x_start: i32,
    x_end: i32,
}

impl EnginePart {
    fn get_adjacent(&self) -> Vec<(i32, i32)> {
        let mut adjacent = Vec::new();
        for x in (self.x_start - 1)..=self.x_end {
            adjacent.push((x, self.y + 1));
            adjacent.push((x, self.y - 1));
        }
        adjacent.push((self.x_start - 1, self.y));
        adjacent.push((self.x_end, self.y));

        adjacent
    }
}

pub fn part1(input: &str) -> usize {
    let (engine_parts, symbol_locations) = parse(input);

    engine_parts
        .iter()
        .filter(|engine_part| is_part_number(engine_part, &symbol_locations))
        .map(|engine_part| engine_part.value)
        .sum()
}

pub fn part2(input: &str) -> usize {
    let (engine_parts, symbol_locations) = parse(input);
    let gear_connections = build_gear_connections(&engine_parts, &symbol_locations);

    gear_connections.values()
        .filter(|parts| parts.len() == 2)
        .map(|parts| parts[0] * parts[1])
        .sum()
}

fn is_part_number(engine_part: &EnginePart, symbols: &HashMap<(i32, i32), char>) -> bool {
    engine_part
        .get_adjacent()
        .iter()
        .any(|coordinate| symbols.contains_key(coordinate))
}

fn build_gear_connections(
    engine_parts: &[EnginePart],
    symbols: &HashMap<(i32, i32), char>,
) -> HashMap<(i32, i32), Vec<usize>> {
    let mut gear_connected_to: HashMap<(i32, i32), Vec<usize>> = HashMap::new();

    for part in engine_parts {
        for adjacent in part.get_adjacent() {
            if !symbols.contains_key(&adjacent) || symbols.get(&adjacent).unwrap() != &'*' {
                continue;
            } 

            gear_connected_to.entry(adjacent).or_default().push(part.value);
        }
    }

    gear_connected_to
}

fn parse(input: &str) -> (Vec<EnginePart>, HashMap<(i32, i32), char>) {
    let engine_parts = parse_engine_parts(input);
    let symbol_locations = parse_symbol_locations(input);

    (engine_parts, symbol_locations)
}

fn parse_engine_parts(input: &str) -> Vec<EnginePart> {
    let part_pattern = Regex::new(r"(\d+)").unwrap();

    input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            part_pattern.captures_iter(row).map(move |capture| {
                let number = capture.get(1).unwrap();
                let x_start = number.start() as i32;
                let x_end = number.end() as i32;
                let number = number.as_str().parse().unwrap();

                EnginePart {
                    value: number,
                    y: y as i32,
                    x_start,
                    x_end,
                }
            })
        })
        .collect()
}

fn parse_symbol_locations(input: &str) -> HashMap<(i32, i32), char> {
    let mut symbol_locations = HashMap::new();
    for (y, row) in input.lines().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            if ch.is_ascii_digit() || ch == '.' {
                continue;
            }
            symbol_locations.insert((x as i32, y as i32), ch);
        }
    }

    symbol_locations
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../examples/day03.txt");

    #[test]
    fn part1_ex() {
        assert_eq!(4361, part1(EXAMPLE));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(467835, part2(EXAMPLE));
    }
}
