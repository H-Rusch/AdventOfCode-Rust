use regex::Regex;

struct Sensor {
    position: (isize, isize),
    closest_beacon: (isize, isize),
    distance: isize,
}

impl Sensor {
    fn new(position: (isize, isize), closest_beacon: (isize, isize)) -> Sensor {
        let (x1, y1) = position;
        let (x2, y2) = closest_beacon;
        let distance = util::manhatten_distance(x1, y1, x2, y2);

        Sensor {
            position,
            closest_beacon,
            distance,
        }
    }
    /// check whether the given point is inside the area of the square by checking if its x-value
    /// is contained in the range of x-values the sensor spans on the corresponding y-value
    fn is_inside(&self, point: (isize, isize)) -> bool {
        // point and square are too far away from each other on the y axis
        if (self.position.1 - point.1).abs() >= self.distance {
            return false;
        }

        let width = (self.distance - (self.position.1 - point.1).abs()).abs();
        let x_range = self.position.0 - width..=(self.position.0 + width);

        x_range.contains(&point.0)
    }
}

pub fn part1(input: &str) -> usize {
    part1_work(input, 2_000_000)
}

fn part1_work(input: &str, y: isize) -> usize {
    let sensors = parse(input);

    let left = sensors
        .iter()
        .map(|s| s.position.0 - s.distance)
        .min()
        .unwrap();
    let right = sensors
        .iter()
        .map(|s| s.position.0 + s.distance)
        .max()
        .unwrap();

    (left..=right)
        .filter(|&x| sensors.iter().any(|sensor| sensor.is_inside((x, y))))
        .count()

}

pub fn part2(input: &str) -> usize {
    0
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

    const INPUT: &str = "\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn part1_ex() {
        assert_eq!(26, part1_work(INPUT, 10));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(0, part2(INPUT));
    }
}
