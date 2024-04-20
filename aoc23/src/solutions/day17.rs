use std::collections::{BinaryHeap, HashMap, HashSet};
use util::grid::{Coordinate, Direction};

#[derive(Eq, PartialEq)]
struct CrucibleState {
    heat_loss: u32,
    crucible: Crucible,
}

impl CrucibleState {
    fn from(heat_loss: u32, crucible: Crucible) -> Self {
        Self {
            heat_loss,
            crucible,
        }
    }
}

// implement ordering, because binary heap is a max-heap by default
impl Ord for CrucibleState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.heat_loss.cmp(&self.heat_loss)
    }
}

impl PartialOrd for CrucibleState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Eq, PartialEq, Clone, Hash)]
struct Crucible {
    position: Coordinate,
    direction: Direction,
    straight_count: u8,
}

impl Crucible {
    fn new(direction: Direction) -> Self {
        Self::from(Coordinate::from(0, 0), direction, 0)
    }

    fn from(position: Coordinate, direction: Direction, straight_count: u8) -> Self {
        Self {
            position,
            direction,
            straight_count,
        }
    }

    fn get_next_states(
        &self,
        blocks: &HashMap<Coordinate, u32>,
        straight_min: u8,
        straight_max: u8,
    ) -> Vec<Crucible> {
        if self.straight_count < straight_min {
            return Some(self.position.step(&self.direction, 1))
                .filter(|next_position| blocks.contains_key(next_position))
                .map(|next_position| {
                    Crucible::from(next_position, self.direction, self.straight_count + 1)
                })
                .into_iter()
                .collect();
        }

        [
            (self.direction, self.straight_count + 1),
            (self.direction.left(), 1),
            (self.direction.right(), 1),
        ]
        .iter()
        .filter(|(_, next_straight_count)| next_straight_count <= &straight_max)
        .map(|(next_direction, next_straight_count)| {
            (
                self.position.step(next_direction, 1),
                next_direction,
                next_straight_count,
            )
        })
        .filter(|(next_position, _, _)| blocks.contains_key(next_position))
        .map(|(next_position, next_direction, next_straight_count)| {
            Crucible::from(next_position, *next_direction, *next_straight_count)
        })
        .collect()
    }

    fn has_reached_goal(&self, goal: &Coordinate, straight_min: u8) -> bool {
        &self.position == goal && self.straight_count >= straight_min
    }
}

pub fn part1(input: &str) -> u32 {
    let blocks = parse(input);
    let goal = find_goal(&blocks);
    lowest_heat_loss(&blocks, goal, 1, 3)
}

pub fn part2(input: &str) -> u32 {
    let blocks = parse(input);
    let goal = find_goal(&blocks);
    lowest_heat_loss(&blocks, goal, 4, 10)
}

fn find_goal(blocks: &HashMap<Coordinate, u32>) -> &Coordinate {
    blocks
        .keys()
        .max_by(|c1, c2| (c1.x, c1.y).cmp(&(c2.x, c2.y)))
        .unwrap()
}

fn lowest_heat_loss(
    blocks: &HashMap<Coordinate, u32>,
    goal: &Coordinate,
    straight_min: u8,
    straight_max: u8,
) -> u32 {
    let mut heap = BinaryHeap::from([
        CrucibleState::from(0, Crucible::new(Direction::Right)),
        CrucibleState::from(0, Crucible::new(Direction::Down)),
    ]);
    let mut visited = HashSet::new();

    while let Some(CrucibleState {
        heat_loss,
        crucible,
    }) = heap.pop()
    {
        if crucible.has_reached_goal(goal, straight_min) {
            return heat_loss;
        }

        if !visited.insert(crucible.clone()) {
            continue;
        }

        crucible
            .get_next_states(blocks, straight_min, straight_max)
            .into_iter()
            .filter(|next_crucible| blocks.contains_key(&next_crucible.position))
            .map(|next_crucible| {
                let next_heat_loss = heat_loss + blocks.get(&next_crucible.position).unwrap();
                CrucibleState::from(next_heat_loss, next_crucible)
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

    const EXAMPLE_1: &str = include_str!("../../examples/day17_1.txt");
    const EXAMPLE_2: &str = include_str!("../../examples/day17_2.txt");

    #[test]
    fn get_next_state_should_return_straight_and_turning() {
        let crucible = Crucible::new(Direction::Right);
        let blocks = HashMap::from([(Coordinate::from(1, 0), 3), (Coordinate::from(0, 1), 5)]);

        let next_crucibles = crucible.get_next_states(&blocks, 0, 3);

        assert_eq!(2, next_crucibles.len());
        assert!(next_crucibles.contains(&Crucible::from(
            Coordinate::from(1, 0),
            Direction::Right,
            1
        )));
        assert!(next_crucibles.contains(&Crucible::from(
            Coordinate::from(0, 1),
            Direction::Down,
            1
        )));
    }

    #[test]
    fn get_next_state_should_only_return_straight_when_not_going_straight_far_enough() {
        let crucible = Crucible::new(Direction::Right);
        let blocks = HashMap::from([(Coordinate::from(1, 0), 3)]);

        let next_crucibles = crucible.get_next_states(&blocks, 1, 3);

        assert_eq!(1, next_crucibles.len());
        assert!(next_crucibles.contains(&Crucible::from(
            Coordinate::from(1, 0),
            Direction::Right,
            1
        )));
    }

    #[test]
    fn get_next_state_should_only_return_turning_going_straight_too_far() {
        let crucible = Crucible::from(Coordinate::from(1, 1), Direction::Right, 3);
        let blocks = HashMap::from([
            (Coordinate::from(2, 1), 3),
            (Coordinate::from(1, 0), 3),
            (Coordinate::from(1, 2), 3),
        ]);
        let next_crucibles = crucible.get_next_states(&blocks, 0, 3);

        assert_eq!(2, next_crucibles.len());
        assert!(next_crucibles.contains(&Crucible::from(Coordinate::from(1, 0), Direction::Up, 1)));
        assert!(next_crucibles.contains(&Crucible::from(
            Coordinate::from(1, 2),
            Direction::Down,
            1
        )));
    }

    #[test]
    fn part1_ex() {
        assert_eq!(102, part1(EXAMPLE_1));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(94, part2(EXAMPLE_1));
        assert_eq!(71, part2(EXAMPLE_2));
    }
}
