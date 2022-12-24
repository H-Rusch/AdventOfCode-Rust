use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

pub fn part1(input: &str) -> u32 {
    let (coordinates, mut blizzards) = parse(input);
    let (start, goal) = find_start_end(&coordinates);

    find_shortest_path(&coordinates, &mut blizzards, start, goal)
}

pub fn part2(input: &str) -> u32 {
    let (coordinates, mut blizzards) = parse(input);
    let (start, goal) = find_start_end(&coordinates);

    // just go from start to end, from end to start and then from start to end.
    let way1 = find_shortest_path(&coordinates, &mut blizzards, start, goal);
    let way2 = find_shortest_path(&coordinates, &mut blizzards, goal, start);
    let way3 = find_shortest_path(&coordinates, &mut blizzards, start, goal);

    way1 + way2 + way3 + 2
}

fn find_shortest_path(
    coordinates: &HashSet<(i32, i32)>,
    blizzards: &mut Vec<(i32, i32, char)>,
    start: (i32, i32),
    goal: (i32, i32),
) -> u32 {
    // Breadth first search

    // lookup which saves the coordinates of all blizzards at a specific time
    let mut blizzard_locations: HashMap<u32, HashSet<(i32, i32)>> =
        HashMap::from([(0, get_blizzard_locations(blizzards))]);

    // state := (time, (x, y))
    let mut queue = VecDeque::from([(0, start)]);
    let mut visited = HashSet::new();

    while let Some((time, (x, y))) = queue.pop_front() {
        if (x, y) == goal {
            return time;
        }
        if visited.contains(&(time, (x, y))) {
            continue;
        }
        visited.insert((time, (x, y)));

        let next_time = time + 1;
        if !blizzard_locations.keys().contains(&next_time) {
            // simulate next step for the blizzards and add coordinates to the map
            *blizzards = simulate_next_blizzards(coordinates, blizzards);
            blizzard_locations.insert(next_time, get_blizzard_locations(blizzards));
        }

        // add states for the possible subsequent positions to the queue
        for (xx, yy) in get_possible_moves(x, y) {
            // next coordinate does not exist or is blocked by a blizzard
            if !coordinates.contains(&(xx, yy))
                || blizzard_locations
                    .get(&next_time)
                    .unwrap()
                    .contains(&(xx, yy))
            {
                continue;
            }
            queue.push_back((next_time, (xx, yy)));
        }
    }

    unreachable!()
}

fn simulate_next_blizzards(
    coordinates: &HashSet<(i32, i32)>,
    blizzards: &[(i32, i32, char)],
) -> Vec<(i32, i32, char)> {
    let width = coordinates.iter().max_by_key(|(x, _)| x).unwrap().0;
    let height = coordinates.iter().max_by_key(|(_, y)| y).unwrap().1 - 1;
    // let mut blizzards = Vec::with_capacity(blizzards.len());

    blizzards
        .iter()
        .map(|&(x, y, d)| {
            let (mut next_x, mut next_y) = match d {
                'v' => (x, y + 1),
                '^' => (x, y - 1),
                '<' => (x - 1, y),
                '>' => (x + 1, y),
                _ => unreachable!(),
            };

            // wrapping around without modulo, because positions start at 1
            if next_x == 0 {
                next_x = width;
            } else if next_x == width + 1 {
                next_x = 1;
            } else if next_y == 0 {
                next_y = height;
            } else if next_y == height + 1 {
                next_y = 1;
            }

            (next_x, next_y, d)
        })
        .collect()
}

fn get_blizzard_locations(blizzards: &[(i32, i32, char)]) -> HashSet<(i32, i32)> {
    blizzards.iter().map(|(x, y, _)| (*x, *y)).collect()
}

fn find_start_end(coordinates: &HashSet<(i32, i32)>) -> ((i32, i32), (i32, i32)) {
    let start = *coordinates.iter().min_by_key(|(_, y)| y).unwrap();
    let end = *coordinates.iter().max_by_key(|(_, y)| y).unwrap();

    (start, end)
}

fn get_possible_moves(x: i32, y: i32) -> impl Iterator<Item = (i32, i32)> {
    [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1), (x, y)].into_iter()
}

#[allow(clippy::type_complexity)]
fn parse(input: &str) -> (HashSet<(i32, i32)>, Vec<(i32, i32, char)>) {
    let mut coordinates = HashSet::new();
    let mut blizzards = Vec::new();

    for (y, line) in input.lines().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            let (x, y) = (x as i32, y as i32);
            match cell {
                '<' | '>' | 'v' | '^' => {
                    coordinates.insert((x, y));
                    blizzards.push((x, y, cell));
                }
                '.' => {
                    coordinates.insert((x, y));
                }
                _ => {}
            }
        }
    }

    (coordinates, blizzards)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../examples/day24.txt");

    #[test]
    fn part1_ex() {
        assert_eq!(18, part1(INPUT));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(54, part2(INPUT));
    }
}
