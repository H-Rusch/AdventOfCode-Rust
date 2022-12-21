use std::collections::HashMap;
use num_complex::Complex;

enum Expression {
    Value(Complex<f64>),
    Add(String, String),
    Sub(String, String),
    Mult(String, String),
    Div(String, String),
}

impl Expression {
    fn calculate(&self, map: &HashMap<&str, Expression>) -> Complex<f64> {
        use Expression::*;

        match self {
            Value(v) => *v,
            Add(s1, s2) => {
                map.get(s1.as_str()).unwrap().calculate(map) + map.get(s2.as_str()).unwrap().calculate(map)
            }
            Sub(s1, s2) => {
                map.get(s1.as_str()).unwrap().calculate(map) - map.get(s2.as_str()).unwrap().calculate(map)
            }
            Mult(s1, s2) => {
                map.get(s1.as_str()).unwrap().calculate(map) * map.get(s2.as_str()).unwrap().calculate(map)
            }
            Div(s1, s2) => {
                map.get(s1.as_str()).unwrap().calculate(map) / map.get(s2.as_str()).unwrap().calculate(map)
            }
        }
    }
}

pub fn part1(input: &str) -> i64 {
    let expression_map = parse(input);

    expression_map.get("root").unwrap().calculate(&expression_map).re as i64
}

pub fn part2(input: &str) -> i64 {
    let mut expression_map = parse(input);
    // unsing imaginary numbers modeled after /u/Anton31Kah's idea: https://www.reddit.com/r/adventofcode/comments/zrav4h/2022_day_21_solutions/j133ko6/
    expression_map.insert("humn", Expression::Value(Complex::new(0.0, 1.0)));

    if let Expression::Add(s1, s2) = expression_map.get("root").unwrap() {
        let mut solve = expression_map.get(s1.as_str()).unwrap().calculate(&expression_map);
        let mut constant = expression_map.get(s2.as_str()).unwrap().calculate(&expression_map);

        if constant.im != 0.0 {
            (solve, constant) = (constant, solve);
        }

        return ((constant.re - solve.re) / solve.im).ceil() as i64;
    }

    unreachable!()
}

fn parse(input: &str) -> HashMap<&str, Expression> {
    let map: HashMap<&str, Vec<&str>> = input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(": ").unwrap();
            let right = right.split_whitespace().collect();
            (left, right)
        })
        .collect();

    let mut expressions = HashMap::with_capacity(map.len());
    for (key, v) in map {
        let expression = if v.len() == 1 {
            Expression::Value(v[0].parse().unwrap())
        } else {
            match v[1] {
                "+" => Expression::Add(v[0].to_string(), v[2].to_string()),
                "-" => Expression::Sub(v[0].to_string(), v[2].to_string()),
                "*" => Expression::Mult(v[0].to_string(), v[2].to_string()),
                "/" => Expression::Div(v[0].to_string(), v[2].to_string()),
                _ => unreachable!(),
            }
        };

        expressions.insert(key, expression);
    }

    expressions
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../examples/day21.txt");

    #[test]
    fn part1_ex() {
        assert_eq!(152, part1(INPUT));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(301, part2(INPUT));
    }
}
