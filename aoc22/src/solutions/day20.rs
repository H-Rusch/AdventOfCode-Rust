use std::collections::VecDeque;

struct CyclingVec {
    values: VecDeque<(usize, i64)>,
}

impl CyclingVec {
    fn move_num(&mut self, index: usize, value: i64) {
        // find index of the value that should be moved
        let pos = self
            .values
            .iter()
            .position(|v| v == &(index, value))
            .unwrap();
        // remove that value from the list by getting it to the front and then popping it
        self.values.rotate_left(pos);
        let (i, n) = self.values.pop_front().unwrap();
        // rotate the values as often to the left as the popped value defined
        let rotation = n.rem_euclid(self.values.len() as i64) as usize;
        self.values.rotate_left(rotation);
        // add the popped ('index', 'value') pair to the list again
        self.values.push_front((i, n));
    }

    fn get(&self, index: usize) -> i64 {
        self.values[index % self.values.len()].1
    }
}

pub fn part1(input: &str) -> i64 {
    let values = parse(input);
    
    solve(&values, 1, 1)
}

pub fn part2(input: &str) -> i64 {
    let values = parse(input);
    
    solve(&values, 811_589_153, 10)
}

fn solve(values: &[i64], key: i64, times: usize) -> i64 {
    // transform 'value' to ('index', 'value') because there are duplicate values in the actual input
    let values: VecDeque<(usize, i64)> = values.iter().map(|n| *n * key).enumerate().collect();
    let mut cycling = CyclingVec { values: values.clone() };

    for _ in 0..times {
        for (i, value) in values.iter() {
            cycling.move_num(*i, *value);
        }
    }

    let zero_pos = cycling.values.iter().position(|(_, v)| *v == 0).unwrap();
    [1000, 2000, 3000]
        .into_iter()
        .map(|n| cycling.get(n + zero_pos))
        .sum()
}

fn parse(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|n| n.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../examples/day20.txt");

    #[test]
    fn part1_ex() {
        assert_eq!(3, part1(INPUT));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(1_623_178_306, part2(INPUT));
    }
}
