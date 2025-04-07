use std::collections::{HashMap, HashSet, VecDeque};

use util::grid::{Coordinate, Direction};

#[derive(PartialEq, Debug, Clone)]
enum Tile {
    Path,
    Slope(Direction),
}

impl Tile {
    fn from(ch: char) -> Option<Self> {
        match ch {
            '.' => Some(Tile::Path),
            '>' | 'v' | '<' | '^' => Some(Tile::Slope(Direction::from(ch))),
            _ => None,
        }
    }
}

trait Adjacent {
    fn get_adjacent(&self, coordinate: &Coordinate) -> Vec<Coordinate>;
    fn get_adjacent_tiles(&self, coordinate: &Coordinate) -> Vec<Coordinate>;
}

struct Part1Adjacent {
    tiles: HashMap<Coordinate, Tile>,
}

impl Adjacent for Part1Adjacent {
    fn get_adjacent(&self, coordinate: &Coordinate) -> Vec<Coordinate> {
        match self.tiles.get(coordinate).unwrap() {
            Tile::Path => self.get_adjacent_tiles(coordinate),
            Tile::Slope(direction) => vec![coordinate.step(direction, 1)],
        }
    }

    fn get_adjacent_tiles(&self, coordinate: &Coordinate) -> Vec<Coordinate> {
        Direction::iter()
            .map(|direction| (direction, coordinate.step(&direction, 1)))
            .filter(|(direction, coord)| {
                if let Some(adjacent) = self.tiles.get(coord) {
                    return match adjacent {
                        Tile::Path => true,
                        Tile::Slope(slope_direction) => slope_direction == direction,
                    };
                }

                false
            })
            .map(|(_, adjacent)| adjacent)
            .collect()
    }
}

struct Part2Adjacent {
    tiles: HashMap<Coordinate, Tile>,
}

impl Adjacent for Part2Adjacent {
    fn get_adjacent(&self, coordinate: &Coordinate) -> Vec<Coordinate> {
        self.get_adjacent_tiles(coordinate)
    }

    fn get_adjacent_tiles(&self, coordinate: &Coordinate) -> Vec<Coordinate> {
        Direction::iter()
            .map(|direction| coordinate.step(&direction, 1))
            .filter(|coord| self.tiles.contains_key(coord))
            .collect()
    }
}

pub fn part1(input: &str) -> usize {
    let tiles = parse(input);
    let adjacent_strategy = Part1Adjacent {
        tiles: tiles.clone(),
    };

    part(tiles, &adjacent_strategy)
}

pub fn part2(input: &str) -> usize {
    let tiles = parse(input);
    let adjacent_strategy = Part2Adjacent {
        tiles: tiles.clone(),
    };

    part(tiles, &adjacent_strategy)
}

fn part(tiles: HashMap<Coordinate, Tile>, adjacent_strategy: &dyn Adjacent) -> usize {
    let start = find_start(&tiles);
    let goal = find_goal(&tiles);

    let condensed_graph = condense_graph(start, goal, adjacent_strategy);
    find_longest_path(&condensed_graph, start, goal)
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
    condensed_graph: &HashMap<Coordinate, Vec<(Coordinate, usize)>>,
    start: Coordinate,
    goal: Coordinate,
) -> usize {
    let mut stack = Vec::from([(start, 0, HashSet::new())]);
    let mut best = 0;

    while let Some((current, length, mut path)) = stack.pop() {
        if current == goal {
            best = best.max(length);
            continue;
        }

        if path.contains(&current) {
            continue;
        }
        path.insert(current);

        for (next, additional_length) in condensed_graph.get(&current).unwrap() {
            stack.push((*next, length + additional_length, path.clone()));
        }
    }

    best
}

fn condense_graph(
    start: Coordinate,
    goal: Coordinate,
    adjacent_strategy: &dyn Adjacent,
) -> HashMap<Coordinate, Vec<(Coordinate, usize)>> {
    let mut condensed_grap: HashMap<Coordinate, Vec<(Coordinate, usize)>> = HashMap::new();
    let mut queue = VecDeque::from([start]);
    let mut visited = HashSet::new();

    while let Some(current) = queue.pop_front() {
        if visited.contains(&current) || current == goal {
            continue;
        }
        visited.insert(current);

        let adjactent = adjacent_strategy.get_adjacent(&current);
        if adjactent.len() == 2 {
            continue;
        }

        let continious_paths_goals = walk_paths(&current, adjacent_strategy);

        queue.extend(continious_paths_goals.iter().map(|(coord, _)| coord));
        condensed_grap.insert(current, continious_paths_goals);
    }

    condensed_grap
}

fn walk_paths(current: &Coordinate, adjacent_strategy: &dyn Adjacent) -> Vec<(Coordinate, usize)> {
    let mut stack: Vec<(Coordinate, usize)> = adjacent_strategy
        .get_adjacent(current)
        .iter()
        .map(|it| (*it, 1))
        .collect();
    let mut visited = HashSet::from([*current]);
    let mut results = Vec::new();
    let previous = current;

    while let Some((current, length)) = stack.pop() {
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current);

        let adjacent = adjacent_strategy.get_adjacent(&current);
        if adjacent.len() != 2 {
            if current != *previous {
                results.push((current, length));
            }
            continue;
        }

        for adj in adjacent {
            stack.push((adj, length + 1));
        }
    }

    results
}

fn parse(input: &str) -> HashMap<Coordinate, Tile> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.char_indices().filter_map(move |(x, ch)| {
                Tile::from(ch).map(|tile| (Coordinate::from(x as i32, y as i32), tile))
            })
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
        assert_eq!(154, part2(EXAMPLE));
    }
}
