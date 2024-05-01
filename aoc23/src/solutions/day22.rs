use regex::Regex;
use std::collections::{HashMap, HashSet};

use util::grid::Coordinate;

type HeightMap = HashMap<Coordinate, (usize, Option<usize>)>;
type BrickMap = HashMap<usize, HashSet<usize>>;

struct Brick {
    index: usize,
    start: Coordinate,
    end: Coordinate,
    z: usize,
    height: usize,
}

impl Brick {
    fn fall(
        &mut self,
        height_map: &mut HeightMap,
        bricks_on_top: &mut BrickMap,
        brick_supported_by: &mut BrickMap,
    ) {
        self.z = self.get_z_after_landing(height_map);

        self.link_with_foundation(height_map, bricks_on_top, brick_supported_by);

        self.update_height_map(height_map);
    }

    fn iter_area(&self) -> impl Iterator<Item = Coordinate> + '_ {
        (self.start.x..=self.end.x)
            .flat_map(|x| (self.start.y..=self.end.y).map(move |y| Coordinate::from(x, y)))
    }

    fn get_z_after_landing(&self, height_map: &HeightMap) -> usize {
        1 + self
            .iter_area()
            .map(|coord| height_map.get(&coord).unwrap_or(&(0, Some(0))).0) // Some(0) means ground as indices of Bricks start at 1
            .max()
            .unwrap()
    }

    fn link_with_foundation(
        &self,
        height_map: &mut HeightMap,
        bricks_on_top: &mut BrickMap,
        brick_supported_by: &mut BrickMap,
    ) {
        self.iter_area().for_each(|coord| {
            if height_map.get(&coord).unwrap_or(&(0, Some(0))).0 != self.z - 1 {
                return;
            }

            let foundation_index = height_map.get(&coord).unwrap_or(&(0, Some(0))).1.unwrap();
            bricks_on_top
                .entry(foundation_index)
                .or_default()
                .insert(self.index);

            brick_supported_by
                .entry(self.index)
                .or_default()
                .insert(foundation_index);
        });
    }

    fn update_height_map(&self, height_map: &mut HeightMap) {
        let new_z = self.z + self.height;
        self.iter_area().for_each(|coord| {
            height_map.insert(coord, (new_z, Some(self.index)));
        });
    }
}

pub fn part1(input: &str) -> usize {
    let mut bricks = parse(input);
    let (bricks_on_top, brick_supported_by) = perform_initial_fall(&mut bricks);

    bricks
        .iter()
        .filter(|brick| {
            can_brick_safely_be_disintegrated(brick, &bricks_on_top, &brick_supported_by)
        })
        .count()
}

pub fn part2(input: &str) -> usize {
    let mut bricks = parse(input);
    let (bricks_on_top, brick_supported_by) = perform_initial_fall(&mut bricks);

    bricks
        .iter()
        .map(|brick| {
            count_potential_falling_bricks(brick.index, &bricks_on_top, &brick_supported_by)
        })
        .sum()
}

fn perform_initial_fall(bricks: &mut [Brick]) -> (BrickMap, BrickMap) {
    let mut height_map = HashMap::new();
    let mut bricks_on_top = HashMap::new();
    let mut brick_supported_by = HashMap::new();

    sort_by_height(bricks);

    bricks
        .iter_mut()
        .for_each(|brick| brick.fall(&mut height_map, &mut bricks_on_top, &mut brick_supported_by));

    (bricks_on_top, brick_supported_by)
}

fn sort_by_height(bricks: &mut [Brick]) {
    bricks.sort_by(|a, b| (a.z, a.height).cmp(&(b.z, b.height)))
}

fn can_brick_safely_be_disintegrated(
    brick: &Brick,
    bricks_on_top: &BrickMap,
    brick_supported_by: &BrickMap,
) -> bool {
    !bricks_on_top.contains_key(&brick.index)
        || bricks_on_top
            .get(&brick.index)
            .unwrap()
            .iter()
            .all(|other_index| brick_supported_by.get(other_index).unwrap().len() > 1)
}

fn count_potential_falling_bricks(
    brick_index: usize,
    bricks_on_top: &BrickMap,
    brick_supported_by: &BrickMap,
) -> usize {
    let mut falling_brick_count = 0;
    perform_recursive_falling(
        brick_index,
        &mut bricks_on_top.clone(),
        &mut brick_supported_by.clone(),
        &mut falling_brick_count,
    );

    falling_brick_count
}

fn perform_recursive_falling(
    brick_index: usize,
    bricks_on_top: &mut BrickMap,
    brick_supported_by: &mut BrickMap,
    falling_count: &mut usize,
) {
    bricks_on_top
        .get(&brick_index)
        .unwrap_or(&HashSet::new())
        .clone()
        .iter()
        .for_each(|&upper_brick| {
            brick_supported_by.entry(upper_brick).and_modify(|e| {
                e.remove(&brick_index);
            });

            if brick_supported_by.get(&upper_brick).unwrap().is_empty() {
                perform_recursive_falling(
                    upper_brick,
                    bricks_on_top,
                    brick_supported_by,
                    falling_count,
                );

                *falling_count += 1;
            }
        });
}

fn parse(input: &str) -> Vec<Brick> {
    let brick_regex = Regex::new(r"(\d+),(\d+),(\d+)~(\d+),(\d+),(\d+)").unwrap();
    input
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let numbers: Vec<usize> = brick_regex
                .captures(line)
                .unwrap()
                .iter()
                .skip(1)
                .map(Option::unwrap)
                .map(|group| group.as_str().parse::<usize>().unwrap())
                .collect();

            let start = Coordinate::from(numbers[0] as i32, numbers[1] as i32);
            let end = Coordinate::from(numbers[3] as i32, numbers[4] as i32);

            Brick {
                index: 1 + index,
                start,
                end,
                z: numbers[2],
                height: numbers[5] - numbers[2],
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../examples/day22.txt");

    #[test]
    fn brick_falling_on_ground() {
        let mut height_map = HashMap::new();
        let mut bricks_on_top = HashMap::new();
        let mut brick_supported_by = HashMap::new();

        let mut brick = Brick {
            index: 1,
            start: Coordinate::from(1, 1),
            end: Coordinate::from(2, 2),
            z: 40,
            height: 10,
        };
        brick.fall(&mut height_map, &mut bricks_on_top, &mut brick_supported_by);

        assert_eq!(1, brick.z);
        brick
            .iter_area()
            .for_each(|coord| assert_eq!(11, height_map.get(&coord).unwrap().0));
    }

    #[test]
    fn brick_falling_on_brick() {
        let mut height_map = HashMap::new();
        let mut bricks_on_top = HashMap::new();
        let mut brick_supported_by = HashMap::new();

        let mut lower_brick = Brick {
            index: 1,
            start: Coordinate::from(0, 1),
            end: Coordinate::from(2, 2),
            z: 40,
            height: 10,
        };
        let mut upper_brick = Brick {
            index: 2,
            start: Coordinate::from(1, 1),
            end: Coordinate::from(3, 3),
            z: 500,
            height: 5,
        };

        lower_brick.fall(&mut height_map, &mut bricks_on_top, &mut brick_supported_by);
        upper_brick.fall(&mut height_map, &mut bricks_on_top, &mut brick_supported_by);

        assert_eq!(1, lower_brick.z);
        assert_eq!(12, upper_brick.z);

        assert_eq!(
            0,
            height_map
                .get(&Coordinate::from(0, 0))
                .unwrap_or(&(0, Some(0)))
                .0
        );
        assert_eq!(11, height_map.get(&Coordinate::from(0, 1)).unwrap().0);
        upper_brick
            .iter_area()
            .for_each(|coord| assert_eq!(17, height_map.get(&coord).unwrap().0));

        assert!(brick_supported_by
            .get(&upper_brick.index)
            .unwrap()
            .contains(&lower_brick.index));
        assert!(bricks_on_top
            .get(&lower_brick.index)
            .unwrap()
            .contains(&upper_brick.index));
    }

    #[test]
    fn part1_ex() {
        assert_eq!(5, part1(EXAMPLE));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(7, part2(EXAMPLE));
    }
}
