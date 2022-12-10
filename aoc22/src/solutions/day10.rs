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

    #[test]
    fn part1_ex() {
        assert_eq!(13140, part1(INPUT));
    }

    const INPUT: &str = "\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";
}
