use std::{
    collections::{HashSet, VecDeque},
    ops::RangeInclusive,
};

pub fn part1(input: &str) -> usize {
    let coordinates = parse(input);

    coordinates
        .iter()
        .map(|&(x, y, z)| {
            let adjacent_count = get_adjacent_to_cube(x, y, z)
                .filter(|adj_coordinate| coordinates.contains(&adj_coordinate))
                .count();
            6 - adjacent_count
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let coordinates = parse(input);
    let (x_range, y_range, z_range) = get_bounds(&coordinates);

    // generate all cubes which can be filled by water
    let water_cubes = fill_water(&coordinates, &x_range, &y_range, &z_range);

    // sum the number of sides of each lava cube which are reached by water
    coordinates
        .iter()
        .map(|&(x, y, z)| {
            get_adjacent_to_cube(x, y, z)
                .filter(|adj| water_cubes.contains(&adj))
                .count()
        })
        .sum()
}

fn fill_water(
    lava: &HashSet<(i32, i32, i32)>,
    x_range: &RangeInclusive<i32>,
    y_range: &RangeInclusive<i32>,
    z_range: &RangeInclusive<i32>,
) -> HashSet<(i32, i32, i32)> {
    let mut water = HashSet::new();
    let mut expanded = VecDeque::from([(*x_range.start(), *y_range.start(), *z_range.start())]);

    while let Some(coordinate) = expanded.pop_front() {
        if water.contains(&coordinate) {
            continue;
        }
        water.insert(coordinate);

        for (xx, yy, zz) in get_adjacent_to_cube(coordinate.0, coordinate.1, coordinate.2) {
            if !lava.contains(&(xx, yy, zz))
                && x_range.contains(&xx)
                && y_range.contains(&yy)
                && z_range.contains(&zz)
            {
                expanded.push_back((xx, yy, zz));
            }
        }
    }

    water
}

fn get_bounds(
    coordinates: &HashSet<(i32, i32, i32)>,
) -> (
    RangeInclusive<i32>,
    RangeInclusive<i32>,
    RangeInclusive<i32>,
) {
    let min_x = coordinates.iter().min_by_key(|(x, _, _)| *x).unwrap().0;
    let max_x = coordinates.iter().max_by_key(|(x, _, _)| *x).unwrap().0;
    let min_y = coordinates.iter().min_by_key(|(_, y, _)| *y).unwrap().1;
    let max_y = coordinates.iter().max_by_key(|(_, y, _)| *y).unwrap().1;
    let min_z = coordinates.iter().min_by_key(|(_, _, z)| *z).unwrap().2;
    let max_z = coordinates.iter().max_by_key(|(_, _, z)| *z).unwrap().2;

    (
        min_x - 1..=max_x + 1,
        min_y - 1..=max_y + 1,
        min_z - 1..=max_z + 1,
    )
}

fn get_adjacent_to_cube(x: i32, y: i32, z: i32) -> impl Iterator<Item = (i32, i32, i32)> {
    [
        (x + 1, y, z),
        (x - 1, y, z),
        (x, y + 1, z),
        (x, y - 1, z),
        (x, y, z + 1),
        (x, y, z - 1),
    ]
    .into_iter()
}

fn parse(input: &str) -> HashSet<(i32, i32, i32)> {
    input
        .lines()
        .map(|line| {
            let mut num_iter = line.split(",");
            let x = num_iter.next().unwrap().parse().unwrap();
            let y = num_iter.next().unwrap().parse().unwrap();
            let z = num_iter.next().unwrap().parse().unwrap();

            (x, y, z)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../examples/day18.txt");

    #[test]
    fn part1_ex() {
        assert_eq!(64, part1(INPUT));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(58, part2(INPUT));
    }
}
