use regex::Regex;
use std::ops::RangeInclusive;

struct Target {
    x_range: RangeInclusive<i32>,
    y_range: RangeInclusive<i32>,
}

impl Target {
    fn unreachable(&self, state: &State) -> bool {
        self.y_range.start() > &state.y
    }
}

struct State {
    x: i32,
    y: i32,
    x_vel: i32,
    y_vel: i32,
}

impl State {
    fn new(x_vel: i32, y_vel: i32) -> State {
        State {
            x: 0,
            y: 0,
            x_vel,
            y_vel,
        }
    }

    fn step(&mut self) {
        self.x += self.x_vel;
        self.y += self.y_vel;

        if self.x_vel > 0 {
            self.x_vel -= 1;
        }

        self.y_vel -= 1;
    }

    fn is_inbounds(&self, target: &Target) -> bool {
        target.x_range.contains(&self.x) && target.y_range.contains(&self.y)
    }

    fn reaches(&mut self, target: &Target) -> bool {
        while !target.unreachable(self) {
            self.step();

            if self.is_inbounds(target) {
                return true;
            }
        }

        false
    }
}

pub fn part1(input: &str) -> u32 {
    let target = parse(input);
    util::triangular(target.y_range.start().unsigned_abs() - 1)
}

pub fn part2(input: &str) -> u32 {
    let target = parse(input);
    let mut count = 0;

    for x_vel in 0..=*target.x_range.end() {
        for y_vel in *target.y_range.start()..target.y_range.start().abs() {
            let mut state = State::new(x_vel, y_vel);

            if state.reaches(&target) {
                count += 1;
            }
        }
    }
    count
}

fn parse(input: &str) -> Target {
    let vals: Vec<i32> = Regex::new(r"(-?\d+)")
        .unwrap()
        .find_iter(input)
        .map(|cap| cap.as_str().parse::<i32>().unwrap())
        .collect();

    Target {
        x_range: vals[0]..=vals[1],
        y_range: vals[2]..=vals[3],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reach() {
        let target = parse("target area: x=20..30, y=-10..-5");

        assert!(State::new(21, -10).reaches(&target));
        assert!(State::new(8, 0).reaches(&target));
        assert!(State::new(30, -6).reaches(&target));
        assert!(State::new(15, -2).reaches(&target));
        assert!(State::new(21, -7).reaches(&target));
        assert!(State::new(6, 0).reaches(&target));
    }

    #[test]
    fn part1_ex() {
        let input = "target area: x=20..30, y=-10..-5";

        assert_eq!(part1(input), 45);
    }

    #[test]
    fn part2_ex() {
        let input = "target area: x=20..30, y=-10..-5";

        assert_eq!(part2(input), 112);
    }
}
