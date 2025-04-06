use std::collections::{HashMap, HashSet};

use util::grid::{Coordinate, Direction};

#[derive(PartialEq, Debug)]
enum Tile {
    Path,
    Forest,
    Slope(Direction),
}

impl Tile {
    fn from(ch: char) -> Self {
        match ch {
            '.' => Tile::Path,
            '#' => Tile::Forest,
            '>' | 'v' | '<' | '^' => Tile::Slope(Direction::from(ch)),
            _ => unreachable!(),
        }
    }
}

pub fn part1(input: &str) -> usize {
    let tiles = parse(input);
    let start = find_start(&tiles);
    let goal = find_goal(&tiles);

    find_longest_path(&tiles, start, goal)
}

pub fn part2(_: &str) -> usize {
    0
}

fn find_start(tiles: &HashMap<Coordinate, Tile>) -> Coordinate {
    *tiles
        .iter()
        .find(|(coord, tile)| coord.y == 0 && tile == &&Tile::Path)
        .unwrap()
        .0
}

fn find_goal(tiles: &HashMap<Coordinate, Tile>) -> Coordinate {
    let max_y = tiles.keys().map(|coord| coord.y).max().unwrap();
    *tiles
        .iter()
        .find(|(coord, tile)| coord.y == max_y && tile == &&Tile::Path)
        .unwrap()
        .0
}

fn find_longest_path(
    tiles: &HashMap<Coordinate, Tile>,
    start: Coordinate,
    goal: Coordinate,
) -> usize {
    let mut stack = Vec::from([(start, HashSet::new())]);
    let mut best = 0;

    while let Some((coordinate, mut path)) = stack.pop() {
        if coordinate == goal {
            best = best.max(path.len());
            continue;
        }
        if path.contains(&coordinate) {
            continue;
        }
        path.insert(coordinate);

        for adjacent in get_adjacent(tiles, &coordinate) {
            stack.push((adjacent, path.clone()));
        }
    }

    best
}

fn get_adjacent(tiles: &HashMap<Coordinate, Tile>, coordinate: &Coordinate) -> Vec<Coordinate> {
    match tiles.get(coordinate).unwrap() {
        Tile::Path => get_adjacent_tiles(tiles, coordinate),
        Tile::Slope(direction) => vec![coordinate.step(direction, 1)],
        Tile::Forest => panic!("lost in forest"),
    }
}

fn get_adjacent_tiles(
    tiles: &HashMap<Coordinate, Tile>,
    coordinate: &Coordinate,
) -> Vec<Coordinate> {
    Direction::iter()
        .map(|direction| (direction, coordinate.step(&direction, 1)))
        .filter(|(direction, coord)| {
            if let Some(adjacent) = tiles.get(coord) {
                return match adjacent {
                    Tile::Forest => false,
                    Tile::Path => true,
                    Tile::Slope(slope) => slope == direction,
                };
            }

            false
        })
        .map(|(_, adjacent)| adjacent)
        .collect()
}

fn parse(input: &str) -> HashMap<Coordinate, Tile> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.char_indices()
                .map(move |(x, ch)| (Coordinate::from(x as i32, y as i32), Tile::from(ch)))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../examples/day23.txt");

    #[test]
    fn part1_ex() {
        assert_eq!(94, part1(EXAMPLE));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(0, part2(EXAMPLE));
    }
}
