use std::collections::HashMap;

use itertools::Itertools;

#[derive(PartialEq)]
enum Tile {
    Wall,
    Sand,
}

pub fn part1(input: &str) -> usize {
    let mut tiles = parse(input);
    // y-coordinate for the wall the furthest down from the top.
    let y_cutoff = tiles.keys().max_by_key(|(_, y | y)| *y).unwrap().1;

    fill_with_sand1(&mut tiles, y_cutoff);

    tiles
        .iter()
        .filter(|(_, state)| **state == Tile::Sand)
        .count()
}

pub fn part2(input: &str) -> usize {
    let mut tiles = parse(input);
    let bottom = tiles.keys().max_by_key(|(_, y | y)| *y).unwrap().1 + 2;

    fill_with_sand2(&mut tiles, bottom);

    tiles
        .iter()
        .filter(|(_, state)| **state == Tile::Sand)
        .count()
}

fn fill_with_sand1(tiles: &mut HashMap<(i32, i32), Tile>, cutoff: i32) {
    loop {
        // spawn sand
        let (mut x, mut y) = (500, 0);

        while let Some((next_x, next_y)) = next_coordinate(x, y, tiles) {
            if next_y >= cutoff {
                return;
            }

            x = next_x;
            y = next_y;
        }

        tiles.insert((x, y), Tile::Sand);
    }
}

fn fill_with_sand2(tiles: &mut HashMap<(i32, i32), Tile>, bottom: i32) {
    while !tiles.contains_key(&(500, 0)) {
        // spawn sand
        let (mut x, mut y) = (500, 0);

        while let Some((next_x, next_y)) = next_coordinate(x, y, tiles) {
            if next_y == bottom {
                break;
            }

            x = next_x;
            y = next_y;
        }

        tiles.insert((x, y), Tile::Sand);
    }
}

/// check down, digonal left and diagonal right. If any of those coordinates is empty, this will be the next 
/// coordinate of the tile of sand in which case this coordinate is returned in a Some.
/// But if none of those is empty, the sand should not change position which is represennted by a returned None.
fn next_coordinate(x: i32, y: i32, tiles: &HashMap<(i32, i32), Tile>) -> Option<(i32, i32)> {
    let new_y = y + 1;

    for new_x in [x, x - 1, x + 1] {
        if !tiles.contains_key(&(new_x, new_y)) {
            return Some((new_x, new_y));
        }
    }

    None
}

fn parse(input: &str) -> HashMap<(i32, i32), Tile> {
    let mut solid_tiles: HashMap<(i32, i32), Tile> = HashMap::new();

    for line in input.lines() {
        let coordinates = line.split(" -> ").map(|coordinate| {
            let (x, y) = coordinate.split_once(',').unwrap();
            (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
        });

        for ((x1, y1), (x2, y2)) in coordinates.clone().zip(coordinates.skip(1)) {
            let (x_min, x_max) = (x1.min(x2), x1.max(x2));
            let (y_min, y_max) = (y1.min(y2), y1.max(y2));

            (x_min..=x_max)
                .cartesian_product(y_min..=y_max)
                .for_each(|coordinate| {
                    solid_tiles.insert(coordinate, Tile::Wall);
                });
        }
    }

    solid_tiles
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../examples/day14.txt");

    #[test]
    fn part1_ex() {
        assert_eq!(24, part1(INPUT));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(93, part2(INPUT));
    }
}
