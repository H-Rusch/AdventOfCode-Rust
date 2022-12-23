use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn part1(input: &str) -> usize {
    let mut elves = parse(input);

    for i in 0..10 {
        elves = iteration(&elves, i);
    }

    count_empty(&elves)
}

pub fn part2(input: &str) -> u32 {
    let mut elves = parse(input);

    let mut i = 0;
    loop {
        let next_elves = iteration(&elves, i);
        i += 1;
        if next_elves == elves {
            return i;
        }

        elves = next_elves;
    }
}

fn iteration(elves: &HashSet<(i32, i32)>, i: u32) -> HashSet<(i32, i32)> {
    let mut new_elves = HashSet::with_capacity(elves.len());

    // create a map of 'desired pos': Vec<'position of elf who wants this position'>
    let mut desired_moves: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
    'loop_elves: for pos in elves.iter() {
        let (x, y) = *pos;
        // check all 8 adjacent positions. If all are empty do nothing.
        if util::get_all_adj_not_checking(x, y).all(|adj| !elves.contains(&adj)) {
            new_elves.insert(*pos);
            continue;
        }

        // consider each of the four directions
        for j in 0..4 {
            let n = (i + j) % 4;
            let directional = get_in_direction(x, y, n);
            if directional.iter().all(|adj| !elves.contains(adj)) {
                desired_moves.entry(directional[1]).or_default().push(*pos);
                continue 'loop_elves;
            }
        }

        // could not find a direction to go to: Don't move
        new_elves.insert(*pos);
    }

    // Check desired moves:
    // If there is only one position in the vector for a desired move, take the desired move.
    // If there are multiple positions in the vector, take all of those positions.
    for (desired, starting) in desired_moves {
        if starting.len() == 1 {
            new_elves.insert(desired);
        } else {
            new_elves.extend(starting.iter());
        }
    }

    new_elves
}

fn get_in_direction(x: i32, y: i32, n: u32) -> [(i32, i32); 3] {
    match n {
        // north
        0 => [(x - 1, y - 1), (x, y - 1), (x + 1, y - 1)],
        // south
        1 => [(x - 1, y + 1), (x, y + 1), (x + 1, y + 1)],
        // west
        2 => [(x - 1, y - 1), (x - 1, y), (x - 1, y + 1)],
        // east
        3 => [(x + 1, y - 1), (x + 1, y), (x + 1, y + 1)],
        _ => unreachable!(),
    }
}

fn count_empty(elves: &HashSet<(i32, i32)>) -> usize {
    let min_x = elves.iter().min_by_key(|pos| pos.0).unwrap().0;
    let max_x = elves.iter().max_by_key(|pos| pos.0).unwrap().0;
    let min_y = elves.iter().min_by_key(|pos| pos.1).unwrap().1;
    let max_y = elves.iter().max_by_key(|pos| pos.1).unwrap().1;

    (min_x..=max_x)
        .cartesian_product(min_y..=max_y)
        .filter(|pos| !elves.contains(pos))
        .count()
}

fn parse(input: &str) -> HashSet<(i32, i32)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, cell)| {
                    if cell == '#' {
                        Some((x as i32, y as i32))
                    } else {
                        None
                    }
                })
                .collect::<HashSet<(i32, i32)>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../examples/day23.txt");

    #[test]
    fn part1_ex() {
        assert_eq!(110, part1(INPUT));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(20, part2(INPUT));
    }
}
