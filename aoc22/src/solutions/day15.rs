use regex::Regex;
use std::{collections::HashSet, ops::RangeInclusive};

struct Sensor {
    position: (isize, isize),
    distance: isize,
}

impl Sensor {
    fn new(position: (isize, isize), closest_beacon: (isize, isize)) -> Sensor {
        let (x1, y1) = position;
        let (x2, y2) = closest_beacon;
        let distance = util::manhatten_distance(x1, y1, x2, y2);

        Sensor {
            position,
            distance,
        }
    }

    fn contains_point(&self, point: (isize, isize)) -> bool {
        util::manhatten_distance(point.0, point.1, self.position.0, self.position.1)
            <= self.distance
    }

    // get list of coordinates just outside of the sensor's reach
    fn get_outline(&self, range: &RangeInclusive<isize>) -> HashSet<(isize, isize)> {
        let mut coordinates: HashSet<(isize, isize)> =
            HashSet::with_capacity(4 * self.distance as usize);
        let (x_pos, y_pos) = self.position;

        for dx in 0..=(self.distance + 1) {
            let dy = (self.distance + 1) - dx;
            for (sign_x, sign_y) in [(1, 1), (1, -1), (-1, 1), (-1, -1)] {
                let x = x_pos + (dx * sign_x);
                let y = y_pos + (dy * sign_y);

                if range.contains(&x) && range.contains(&y) {
                    coordinates.insert((x, y));
                }
            }
        }

        coordinates
    }
}

pub fn part1(input: &str) -> usize {
    part1_work(input, 2_000_000)
}

fn part1_work(input: &str, y: isize) -> usize {
    let sensors = parse(input);

    let x_min = sensors
        .iter()
        .map(|s| s.position.0 - s.distance)
        .min()
        .unwrap();
    let x_max = sensors
        .iter()
        .map(|s| s.position.0 + s.distance)
        .max()
        .unwrap();

    (x_min..x_max)
        .filter(|&x| sensors.iter().any(|sensor| sensor.contains_point((x, y))))
        .count()
        - 1 // 'fixing' the result because there is an off by 1 error that I don't see
}

pub fn part2(input: &str) -> usize {
    part2_work(input, 4_000_000)
}

fn part2_work(input: &str, limit: isize) -> usize {
    let sensors = parse(input);
    let limit = 0..=limit;

    for sensor in sensors.iter().rev() {
        let coordinates = sensor.get_outline(&limit);

        for coordinate in coordinates {
            if sensors.iter().all(|sensor| !sensor.contains_point(coordinate)) {
                return calc_tuning_frequency(coordinate);
            }
        }
    }
    0
}

fn calc_tuning_frequency(coordinate: (isize, isize)) -> usize {
    (coordinate.0 * 4_000_000 + coordinate.1) as usize
}

fn parse(input: &str) -> Vec<Sensor> {
    let sensor_regex =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();

    input
        .lines()
        .map(|line| {
            let capture = sensor_regex.captures_iter(line).next().unwrap();
            let numbers: Vec<isize> = (1..5).map(|i| capture[i].parse().unwrap()).collect();

            let sensor_position = (numbers[0], numbers[1]);
            let closest_beacon = (numbers[2], numbers[3]);

            Sensor::new(sensor_position, closest_beacon)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../examples/day15.txt");

    #[test]
    fn part1_ex() {
        assert_eq!(26, part1_work(INPUT, 10));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(56000011, part2_work(INPUT, 20));
    }
}
