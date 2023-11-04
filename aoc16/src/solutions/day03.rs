pub fn part1(input: &str) -> usize {
    parse(input)
        .iter()
        .filter(|&values| is_valid_triangle(values))
        .count()
}

pub fn part2(input: &str) -> usize {
    parse_vertially(input)
        .iter()
        .filter(|&values| is_valid_triangle(values))
        .count()
}

fn is_valid_triangle(values: &[u32]) -> bool {
    let sum: u32 = values.iter().sum();

    values.iter().all(|&value| sum - value > value)
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

fn parse_vertially(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .step_by(3)
        .zip(input.lines().skip(1).step_by(3))
        .zip(input.lines().skip(2).step_by(3))
        .flat_map(|((a, b), c)| {
            a.split_whitespace()
                .zip(b.split_whitespace())
                .zip(c.split_whitespace())
                .map(|((x, y), z)| {
                    vec![x, y, z]
                        .into_iter()
                        .map(|n| n.parse::<u32>().unwrap())
                        .collect()
                })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_triangle_test() {
        assert!(!is_valid_triangle(&[5, 10, 25]));
        assert!(is_valid_triangle(&[3, 4, 5]));
        assert!(is_valid_triangle(&[6, 10, 8]));
    }
}
