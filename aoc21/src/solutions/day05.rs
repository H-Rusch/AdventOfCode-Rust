use regex::Regex;
use std::collections::HashMap;

struct Line {
    start: (i32, i32),
    end: (i32, i32),
}

impl Line {
    fn is_straight(&self) -> bool {
        self.start.0 == self.end.0 || self.start.1 == self.end.1
    }

    fn points_between(&self) -> Vec<(i32, i32)> {
        let mut points: Vec<(i32, i32)> = Vec::new();

        let dx = self.start.0.abs_diff(self.end.0);
        let sign_x = (self.end.0 - self.start.0).signum();
        let dy = self.start.1.abs_diff(self.end.1);
        let sign_y = (self.end.1 - self.start.1).signum();

        for i in 0..=dx.max(dy) as i32 {
            points.push((self.start.0 + sign_x * i, self.start.1 + sign_y * i));
        }

        points
    }
}

pub fn part1(input: &str) -> usize {
    let lines = parse(input);
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();

    for line in lines {
        if line.is_straight() {
            for point in line.points_between() {
                *map.entry(point).or_insert(0) += 1;
            }
        }
    }

    map.values().filter(|v| *v > &1).count()
}

pub fn part2(input: &str) -> usize {
    let lines = parse(input);
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();

    for line in lines {
        for point in line.points_between() {
            *map.entry(point).or_insert(0) += 1;
        }
    }

    map.values().filter(|v| *v > &1).count()
}

fn parse(input: &str) -> Vec<Line> {
    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();

    input
        .lines()
        .map(|line| {
            let capture = re.captures_iter(line).next().unwrap();
            let coords: Vec<i32> = (1..5).map(|i| capture[i].parse().unwrap()).collect();

            let start = (coords[0], coords[1]);
            let end = (coords[2], coords[3]);

            Line { start, end }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex() {
        let input = "\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

        assert_eq!(part1(input), 5);
    }

    #[test]
    fn part2_ex() {
        let input = "\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

        assert_eq!(part2(input), 12);
    }
}
