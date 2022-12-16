use std::collections::HashSet;

pub fn part1(input: &str) -> i32 {
    let values = parse(input);
    let cycles = HashSet::from([20, 60, 100, 140, 180, 220]);

    let mut x = 1;
    let mut cycle = 1;
    let mut result = 0;

    for value in values {
        if cycles.contains(&cycle) {
            result += x * cycle;
        }

        cycle += 1;
        x += value;
    }
    result
}

pub fn part2(input: &str) -> &'static str {
    let values = parse(input);
    let mut output = String::new();

    let mut x = 1;
    let mut cycle = 1;

    for value in values {
        let pixel_position = (cycle % 40) - 1;
        let sprite = (x - 1)..=(x + 1);
        output += if sprite.contains(&pixel_position) { "█ " } else { "  " };
        
        if (cycle) % 40 == 0 {
            output += "\n";
        }

        cycle += 1;
        x += value;
    }
    
    println!("{}", output);
    "Printed to stdout"
}

fn parse(input: &str) -> Vec<i32> {
    // When transforming the words into 0, the resulting numbers can be added to x each step, where each step represents a cycle.
    // Idea taken from: https://www.reddit.com/r/adventofcode/comments/zhjfo4/2022_day_10_solutions/izmspl7/ 
    input
        .split_whitespace()
        .map(|element| if let Ok(v) = element.parse() { v } else { 0 })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const INPUT: &str = include_str!("../../examples/day10.txt");

    #[test]
    fn part1_ex() {
        assert_eq!(13140, part1(INPUT));
    }
}
