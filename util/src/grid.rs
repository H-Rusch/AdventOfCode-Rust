use std::ops::Range;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Right,
    Up,
    Left,
    Down,
}

impl Direction {
    pub fn from(c: char) -> Self {
        match c.to_ascii_uppercase() {
            'R' => Direction::Right,
            'U' => Direction::Up,
            'L' => Direction::Left,
            'D' => Direction::Down,
            _ => unreachable!(),
        }
    }

    pub fn turn_right(&mut self) {
        *self = match self {
            Direction::Right => Self::Down,
            Direction::Down => Self::Left,
            Direction::Left => Self::Up,
            Direction::Up => Self::Right,
        }
    }

    pub fn turn_left(&mut self) {
        *self = match self {
            Direction::Right => Self::Up,
            Direction::Up => Self::Left,
            Direction::Left => Self::Down,
            Direction::Down => Self::Right,
        }
    }

    pub fn turn_around(&mut self) {
        *self = match self {
            Direction::Right => Self::Left,
            Direction::Left => Self::Right,
            Direction::Up => Self::Down,
            Direction::Down => Self::Up,
        }
    }
}

/// Two dimensional coordinate
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    pub fn from(x: i32, y: i32) -> Self {
        Coordinate { x, y }
    }

    pub fn manhatten_distance(&self, other: &Coordinate) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    pub fn step(&self, direction: &Direction, steps: u32) -> Self {
        let steps = steps as i32;
        match direction {
            Direction::Right => Coordinate::from(self.x + steps, self.y),
            Direction::Left => Coordinate::from(self.x - steps, self.y),
            Direction::Up => Coordinate::from(self.x, self.y - steps),
            Direction::Down => Coordinate::from(self.x, self.y + steps),
        }
    }

    pub fn step_in_bounds(
        &self,
        direction: &Direction,
        steps: u32,
        bounds: &Bounds,
    ) -> Option<Self> {
        let coordinate = self.step(direction, steps);
        (bounds.x_values.contains(&coordinate.x) && bounds.y_values.contains(&coordinate.y))
            .then_some(coordinate)
    }

    pub fn get_adjacent(&self) -> Vec<Coordinate> {
        [
            Direction::Right,
            Direction::Down,
            Direction::Up,
            Direction::Left,
        ]
        .iter()
        .map(|direction| self.step(direction, 1))
        .collect()
    }
}

pub struct Bounds {
    x_values: Range<i32>,
    y_values: Range<i32>,
}

impl Bounds {
    pub fn from(x_values: Range<i32>, y_values: Range<i32>) -> Self {
        Bounds { x_values, y_values }
    }

    pub fn contains(&self, coordinate: &Coordinate) -> bool {
        self.x_values.contains(&coordinate.x) && self.y_values.contains(&coordinate.y)
    }
}

/// Generate orthogonally adjacent coordinates to the given one.
/// Performs checks, so generated coordinates are inside the grid.
pub fn get_adjacent(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let y = y as i32;
    let x = x as i32;
    let width = width as i32;
    let height = height as i32;

    let values = vec![
        (y > 0, (x, y - 1)),
        (x > 0, (x - 1, y)),
        (y < height - 1, (x, y + 1)),
        (x < width - 1, (x + 1, y)),
    ];

    filter_for_condition(values)
}

/// Generate orthogonally and digonally adjacent coordinates to the given one.
/// Performs checks, so generated coordinates are inside the grid.
pub fn get_adjacent_with_diag(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let y = y as i32;
    let x = x as i32;
    let width = width as i32;
    let height = height as i32;

    let values = vec![
        (x > 0 && y > 0, (x - 1, y - 1)),
        (x > 0, (x - 1, y)),
        (x > 0 && y < height - 1, (x - 1, y + 1)),
        (y > 0, (x, y - 1)),
        (y < height - 1, (x, y + 1)),
        (x < width - 1 && y > 0, (x + 1, y - 1)),
        (x < width - 1, (x + 1, y)),
        (x < width - 1 && y < height - 1, (x + 1, y + 1)),
    ];

    filter_for_condition(values)
}

fn filter_for_condition(values: Vec<(bool, (i32, i32))>) -> impl Iterator<Item = (usize, usize)> {
    values.into_iter().filter_map(|(cond, (dx, dy))| {
        if cond {
            Some((dx as usize, dy as usize))
        } else {
            None
        }
    })
}

/// Generate orthogonally adjacent coordinates to the given coordinate without checking for bounds of the grid.
pub fn get_adj_not_checking(x: i32, y: i32) -> impl Iterator<Item = (i32, i32)> {
    [(x, y - 1), (x - 1, y), (x + 1, y), (x, y + 1)].into_iter()
}

/// Generate orthogonally and digonally adjacent coordinates to the given coordinate without checking for bounds of the grid.
pub fn get_all_adj_not_checking(x: i32, y: i32) -> impl Iterator<Item = (i32, i32)> {
    [
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
    .into_iter()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_out_of_bounds() {
        let i = get_adjacent(0, 5, 10, 5);

        assert_eq!(2, i.count());
    }

    #[test]
    fn diag_all() {
        let left_edge = get_adjacent_with_diag(0, 2, 10, 10);
        let right_edge = get_adjacent_with_diag(9, 2, 10, 10);
        let middle = get_adjacent_with_diag(5, 5, 10, 10);
        let corner = get_adjacent_with_diag(9, 9, 10, 10);
        let top_edge = get_adjacent_with_diag(2, 0, 10, 10);
        let bottom_edge = get_adjacent_with_diag(2, 9, 10, 10);

        assert_eq!(left_edge.count(), 5);
        assert_eq!(right_edge.count(), 5);
        assert_eq!(middle.count(), 8);
        assert_eq!(corner.count(), 3);
        assert_eq!(top_edge.count(), 5);
        assert_eq!(bottom_edge.count(), 5);
    }

    #[test]
    fn stepping_inside_bounds_returns_correct_values() {
        let coordinate = Coordinate::from(1, 1);
        let bounds = Bounds::from(0..3, 0..3);

        let next = coordinate.step_in_bounds(&Direction::Up, 1, &bounds);
        assert!(next.is_some());
        assert_eq!(next.unwrap(), Coordinate::from(1, 0));

        let next = coordinate.step_in_bounds(&Direction::Down, 1, &bounds);
        assert!(next.is_some());
        assert_eq!(next.unwrap(), Coordinate::from(1, 2));

        let next = coordinate.step_in_bounds(&Direction::Right, 1, &bounds);
        assert!(next.is_some());
        assert_eq!(next.unwrap(), Coordinate::from(2, 1));

        let next = coordinate.step_in_bounds(&Direction::Left, 1, &bounds);
        assert!(next.is_some());
        assert_eq!(next.unwrap(), Coordinate::from(0, 1));
    }

    #[test]
    fn stepping_over_bounds_returns_none() {
        let coordinate = Coordinate::from(1, 1);
        let bounds = Bounds::from(0..3, 0..3);

        let next = coordinate.step_in_bounds(&Direction::Up, 2, &bounds);
        assert!(next.is_none());
        let next = coordinate.step_in_bounds(&Direction::Down, 2, &bounds);
        assert!(next.is_none());
        let next = coordinate.step_in_bounds(&Direction::Right, 2, &bounds);
        assert!(next.is_none());
        let next = coordinate.step_in_bounds(&Direction::Left, 2, &bounds);
        assert!(next.is_none());
    }
}
