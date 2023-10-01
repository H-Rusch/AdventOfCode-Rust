use std::collections::HashSet;

struct Seafloor {
    eastwards: HashSet<(usize, usize)>,
    southwards: HashSet<(usize, usize)>,
    right_edge: usize,
    bottom_edge: usize,
}

impl Seafloor {
    fn new(eastwards: HashSet<(usize, usize)>, southwards: HashSet<(usize, usize)>, right_edge: usize, bottom_edge: usize) -> Seafloor {
        Seafloor { eastwards, southwards, right_edge, bottom_edge}
    }

    fn next_state(&self) -> Seafloor {
        let next_east = self.move_east();
        let next_south = self.move_south(&next_east);

        Seafloor::new(next_east, next_south, self.right_edge, self.bottom_edge)
    }

    fn move_east(&self) -> HashSet<(usize, usize)> {
        self.eastwards.iter()
            .map(|&coordinate| {
                match self.check_east_occupied(coordinate) {
                    false => self.get_east(coordinate),
                    true => coordinate
                }
            })
            .collect()
    }

    fn check_east_occupied(&self, coordinate: (usize, usize)) -> bool {
        let east = self.get_east(coordinate);
        self.eastwards.contains(&east) || self.southwards.contains(&east)
    }

    fn get_east(&self, (x, y): (usize, usize)) -> (usize, usize) {
        ((x + 1) % self.right_edge, y)
    }

    fn move_south(&self, next_east: &HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {
        self.southwards.iter()
            .map(|&coordinate| {
                match self.check_south_occupied(coordinate, next_east) {
                    false => self.get_south(coordinate),
                    true => coordinate
                }
            })
            .collect()
    }

    fn check_south_occupied(&self, coordinate: (usize, usize), next_east: &HashSet<(usize, usize)>) -> bool {
        let south = self.get_south(coordinate);
        next_east.contains(&south) || self.southwards.contains(&south)
    }

    fn get_south(&self, (x, y): (usize, usize)) -> (usize, usize) {
        (x, (y + 1) % self.bottom_edge)
    }
}

impl PartialEq for Seafloor {
    fn eq(&self, other: &Self) -> bool {
        self.bottom_edge == other.bottom_edge && self.right_edge == other.right_edge &&  
        self.eastwards.len() == other.eastwards.len() && 
        self.southwards.len() == other.southwards.len() && 
        self.eastwards.iter().all(|&coordinate| other.eastwards.contains(&coordinate)) &&
        self.southwards.iter().all(|&coordinate| other.southwards.contains(&coordinate))
    }
}

pub fn part1(input: &str) -> usize {
    let mut seafloor = parse(input);
    let mut count = 0;

    loop {
        let next_seafloor = seafloor.next_state();
        count += 1;

        if next_seafloor == seafloor {
            break;
        }

        seafloor = next_seafloor;
    }

    count
}

pub fn part2(_: &str) -> &'static str {
    "All done :D"
}

fn parse(input: &str) -> Seafloor {
    let mut eastwards = HashSet::new();
    let mut southwards = HashSet::new();
    let (mut max_x, mut max_y) = (0, 0);


    for (y, line) in input.lines().enumerate() {
        max_y = max_y.max(y);
        for (x, cell) in line.chars().enumerate() {
            max_x = max_x.max(x);
            match cell {
                '>' => {eastwards.insert((x, y));}
                'v' => {southwards.insert((x, y));}
                _ => {}
            }
        }
    }

    Seafloor::new(eastwards, southwards, max_x + 1, max_y + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_MOVEMENT: &str = include_str!("../../examples/day25_simple_movement.txt");
    const WRAP_AROUNT: &str = include_str!("../../examples/day25_wrap_around.txt");
    const REACHING_STABLE_STATE: &str = include_str!("../../examples/day25_reaching_stable_state.txt");
    const INPUT_EXAMPLE: &str = include_str!("../../examples/day25_example.txt");

    #[test]
    fn next_state_test() {
        let seafloor = parse(SIMPLE_MOVEMENT);
        let next_seafloor = seafloor.next_state();

        assert!(seafloor.eastwards.contains(&(1, 1)));
        assert!(seafloor.eastwards.contains(&(7, 2)));
        assert!(seafloor.southwards.contains(&(7, 1)));
        assert!(seafloor.southwards.contains(&(2, 1)));

        assert_eq!(2, next_seafloor.eastwards.len());
        assert_eq!(2, next_seafloor.southwards.len());
        assert!(next_seafloor.eastwards.contains(&(1, 1)));
        assert!(next_seafloor.eastwards.contains(&(8, 2)));
        assert!(next_seafloor.southwards.contains(&(7, 2)));
        assert!(next_seafloor.southwards.contains(&(2, 2)));
    }

    #[test]
    fn wrap_around_test() {
        let seafloor = parse(WRAP_AROUNT);

        assert!(seafloor.eastwards.contains(&(6, 0)));
        assert!(seafloor.eastwards.contains(&(0, 1)));
        assert!(seafloor.eastwards.contains(&(6, 1)));
        assert!(seafloor.southwards.contains(&(1, 1)));

        let next_seafloor = seafloor.next_state();
        assert!(next_seafloor.eastwards.contains(&(0, 0)));
        assert!(next_seafloor.eastwards.contains(&(0, 1)));
        assert!(next_seafloor.eastwards.contains(&(6, 1)));
        assert!(next_seafloor.southwards.contains(&(1, 0)));
    }

    #[test]
    fn reaching_equal_state() {
        let seafloor = parse(REACHING_STABLE_STATE);
        let next_seafloor = seafloor.next_state();

        assert!(seafloor == next_seafloor);
    }

    #[test]
    fn part1_ex() {
        assert_eq!(58, part1(INPUT_EXAMPLE));
    }
}
