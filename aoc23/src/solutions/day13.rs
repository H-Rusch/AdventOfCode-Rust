struct Pattern {
    vertical_lines: Vec<String>,
    horizontal_lines: Vec<String>,
}

enum Reflection {
    Vertical(usize),
    Horizontal(usize),
}

pub fn part1(input: &str) -> usize {
    solve(&parse(input), 0)
}

pub fn part2(input: &str) -> usize {
    solve(&parse(input), 1)
}

fn solve(patterns: &[Pattern], tolerance: usize) -> usize {
    patterns
        .iter()
        .map(|pattern| find_reflection(pattern, tolerance))
        .map(|reflection| match reflection {
            Some(Reflection::Vertical(x)) => x * 100,
            Some(Reflection::Horizontal(y)) => y,
            None => unreachable!(),
        })
        .sum()
}

fn find_reflection(pattern: &Pattern, tolerance: usize) -> Option<Reflection> {
    find_horizontal_reflection(&pattern.vertical_lines, tolerance)
        .or_else(|| find_vertical_reflection(&pattern.horizontal_lines, tolerance))
}

fn find_horizontal_reflection(vertical_lines: &[String], tolerance: usize) -> Option<Reflection> {
    find_reflection_index(vertical_lines, tolerance).map(Reflection::Horizontal)
}

fn find_vertical_reflection(horizontal_lines: &[String], tolerance: usize) -> Option<Reflection> {
    find_reflection_index(horizontal_lines, tolerance).map(Reflection::Vertical)
}

fn find_reflection_index(lines: &[String], tolerance: usize) -> Option<usize> {
    (1..lines.len()).find(|&i| is_symetrical(lines, i - 1, i, tolerance))
}

fn is_symetrical(lines: &[String], mut left: usize, mut right: usize, tolerance: usize) -> bool {
    let mut difference_count = 0;
    loop {
        difference_count += lines[left]
            .chars()
            .zip(lines[right].chars())
            .filter(|(ch1, ch2)| ch1 != ch2)
            .count();

        if left == 0 || right == lines.len() - 1 {
            break;
        }

        left -= 1;
        right += 1;
    }

    tolerance == difference_count
}

fn parse(input: &str) -> Vec<Pattern> {
    input
        .split("\n\n")
        .map(|chunk| {
            let horizontal_lines = chunk.lines().map(|line| line.to_string()).collect();

            let width = chunk.lines().peekable().peek().unwrap().len();
            let vertical_lines = (0..width)
                .map(|x| {
                    chunk
                        .lines()
                        .flat_map(|line| line.chars())
                        .skip(x)
                        .step_by(width)
                        .collect::<String>()
                })
                .collect();

            Pattern {
                vertical_lines,
                horizontal_lines,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../examples/day13.txt");

    #[test]
    fn part1_ex() {
        assert_eq!(405, part1(EXAMPLE));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(400, part2(EXAMPLE));
    }
}
