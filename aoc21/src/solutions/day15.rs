use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    pos: (usize, usize),
}

// implement so the heap becomes a min heap
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part1(input: &str) -> u32 {
    let grid = parse(input);
    lowest_risk(&grid, (grid[0].len() - 1, grid.len() - 1))
}

pub fn part2(input: &str) -> u32 {
    let mut grid = parse(input);
    larger_area(&mut grid);
    lowest_risk(&grid, (grid[0].len() - 1, grid.len() - 1))
}

fn lowest_risk(grid: &Vec<Vec<u32>>, goal: (usize, usize)) -> u32 {
    let height = grid.len();
    let width = grid[0].len();
    let mut heap = BinaryHeap::new();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    heap.push(State {
        cost: 0,
        pos: (0, 0),
    });

    while let Some(State { cost, pos }) = heap.pop() {
        if pos == goal {
            return cost;
        }
        if seen.contains(&pos) {
            continue;
        }
        seen.insert(pos);

        for (dx, dy) in util::get_adjacent(pos.0, pos.1, width, height) {
            if !seen.contains(&(dx, dy)) {
                heap.push(State {
                    cost: cost + grid[dy][dx],
                    pos: (dx, dy),
                });
            }
        }
    }

    unreachable!()
}

fn larger_area(grid: &mut Vec<Vec<u32>>) {
    let original_height = grid.len();
    let original_width = grid[0].len();

    // add five rows
    for _ in 0..4 * original_height {
        let v: Vec<u32> = grid[grid.len() - original_height]
            .iter()
            .map(calc_new_val)
            .collect();
        grid.push(v);
    }

    // fill all rows to correct length
    for row in grid.iter_mut() {
        for _ in 0..4 * original_width {
            row.push(calc_new_val(&row[row.len() - original_width]));
        }
    }
}

fn calc_new_val(n: &u32) -> u32 {
    let val = n + 1;
    if val > 9 {
        val - 9
    } else {
        val
    }
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex() {
        let input = "\
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

        assert_eq!(part1(input), 40);
    }

    #[test]
    fn part2_ex() {
        let input = "\
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

        assert_eq!(part2(input), 315);
    }
}
