pub enum Direction {
    RIGHT,
    UP,
    LEFT,
    DOWN,
}

impl Direction {
    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::RIGHT => Self::DOWN,
            Direction::DOWN => Self::LEFT,
            Direction::LEFT => Self::UP,
            Direction::UP => Self::RIGHT,
        }
    }

    pub fn turn_left(&self) -> Direction {
        match self {
            Direction::RIGHT => Self::UP,
            Direction::UP => Self::LEFT,
            Direction::LEFT => Self::DOWN,
            Direction::DOWN => Self::RIGHT,
        }
    }

    pub fn turn_around(&self) -> Direction {
        match self {
            Direction::RIGHT => Self::LEFT,
            Direction::LEFT => Self::RIGHT,
            Direction::UP => Self::DOWN,
            Direction::DOWN => Self::UP,
        }
    }
}

/// Two dimensional coordinate
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    pub fn manhatten_distance(&self, other: &Coordinate) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    pub fn step(&mut self, direction: &Direction, steps: u32) {
        let steps = steps as i32;
        match direction {
            Direction::RIGHT => self.x += steps,
            Direction::LEFT => self.x -= steps,
            Direction::UP => self.y += steps,
            Direction::DOWN => self.y -= steps,
        }
    }
}

impl Default for Coordinate {
    fn default() -> Self {
        Coordinate { x: 0, y: 0 }
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
}
