use std::collections::{BinaryHeap, HashMap, HashSet};
use util::grid::{Coordinate, Direction};

const STRAIGHT_MAX: u8 = 3;

#[derive(Eq, PartialEq)]
struct HeapKey {
    heat_loss: u32,
    crucible: CrucibleState,
}

impl HeapKey {
    fn from(heat_loss: u32, crucible: CrucibleState) -> Self {
        Self {
            heat_loss,
            crucible,
        }
    }
}

// implement ordering, because binary heap is a max-heap by default
impl Ord for HeapKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.heat_loss.cmp(&self.heat_loss)
    }
}

impl PartialOrd for HeapKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct CrucibleState {
    position: Coordinate,
    direction: Direction,
    straight_count: u8,
}

impl CrucibleState {
    fn new() -> Self {
        Self::from(Coordinate::from(0, 0), Direction::Right, 0)
    }

    fn from(position: Coordinate, direction: Direction, straight_count: u8) -> Self {
        Self {
            position,
            direction,
            straight_count,
        }
    }

    fn get_next_states(&self, blocks: &HashMap<Coordinate, u32>) -> Vec<CrucibleState> {
        [
            self.direction,
            self.direction.left(),
            self.direction.right(),
        ]
        .iter()
        .map(|next_direction| (next_direction, self.position.step(next_direction, 1)))
        .filter(|(_, next_position)| blocks.contains_key(next_position))
        .map(|(next_direction, next_position)| {
            let next_straight_count = if *next_direction == self.direction {
                self.straight_count + 1
            } else {
                1
            };
            (next_straight_count, next_direction, next_position)
        })
        .filter(|(next_straight_count, _, _)| *next_straight_count <= STRAIGHT_MAX)
        .map(|(next_straight_count, next_direction, next_position)| {
            CrucibleState::from(next_position, *next_direction, next_straight_count)
        })
        .collect()
    }

    fn has_reached_goal(&self, goal: &Coordinate) -> bool {
        &self.position == goal
    }
}

pub fn part1(input: &str) -> u32 {
    let blocks = parse(input);
    let goal = blocks
        .keys()
        .max_by(|c1, c2| (c1.x, c1.y).cmp(&(c2.x, c2.y)))
        .unwrap();
    lowest_heat_loss(&blocks, goal)
}

pub fn part2(input: &str) -> usize {
    0
}

fn lowest_heat_loss(blocks: &HashMap<Coordinate, u32>, goal: &Coordinate) -> u32 {
    let mut heap = BinaryHeap::from([HeapKey::from(0, CrucibleState::new())]);
    let mut visited = HashSet::new();

    while let Some(HeapKey {
        heat_loss,
        crucible,
    }) = heap.pop()
    {
        if crucible.has_reached_goal(goal) {
            return heat_loss;
        }

        if visited.contains(&crucible) {
            continue;
        }
        visited.insert(crucible.clone());

        crucible
            .get_next_states(blocks)
            .into_iter()
            .filter(|next_crucible| blocks.contains_key(&next_crucible.position))
            .map(|next_crucible| {
                let next_heat_loss = heat_loss + blocks.get(&next_crucible.position).unwrap();
                HeapKey::from(next_heat_loss, next_crucible)
            })
            .for_each(|state| heap.push(state));
    }

    unreachable!()
}

fn parse(input: &str) -> HashMap<Coordinate, u32> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.char_indices().map(move |(x, ch)| {
                (
                    Coordinate::from(x as i32, y as i32),
                    ch.to_digit(10).unwrap(),
                )
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../examples/day17.txt");

    #[test]
    fn crucible_next_state_test() {
        let crucible = CrucibleState::new();
        let blocks = HashMap::from([(Coordinate::from(1, 0), 3), (Coordinate::from(0, 1), 5)]);

        let next_crucibles = crucible.get_next_states(&blocks);

        println!("{:?}", next_crucibles);
        assert_eq!(2, next_crucibles.len());
        assert!(next_crucibles.contains(&CrucibleState::from(
            Coordinate::from(1, 0),
            Direction::Right,
            1
        )));
        assert!(next_crucibles.contains(&CrucibleState::from(
            Coordinate::from(0, 1),
            Direction::Down,
            1
        )));
    }

    #[test]
    fn part1_ex() {
        assert_eq!(102, part1(EXAMPLE));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(0, part2(EXAMPLE));
    }
}
