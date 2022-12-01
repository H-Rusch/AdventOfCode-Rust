pub mod dlin;

// numbers
pub fn triangular(n: u32) -> u32 {
    n * (n + 1) / 2
}

pub fn mean(arr: &[i32]) -> f64 {
    arr.iter().sum::<i32>() as f64 / arr.len() as f64
}

pub fn median(arr: &[i32]) -> f64 {
    let mut sorted = arr.to_vec();
    sorted.sort();

    let mid = sorted.len() / 2;

    if sorted.len() % 2 == 0 {
        mean(&sorted[(mid - 1)..(mid + 1)])
    } else {
        sorted[mid] as f64
    }
}

// coordinates
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
    [
        (y > 0, (x, y - 1)),
        (x > 0, (x - 1, y)),
        (y < height - 1, (x, y + 1)),
        (x < width - 1, (x + 1, y)),
    ]
    .into_iter()
    .filter_map(|(cond, (dx, dy))| {
        if cond {
            Some((dx as usize, dy as usize))
        } else {
            None
        }
    })
}

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
    [
        (x > 0 && y > 0, (x - 1, y - 1)),
        (x > 0, (x - 1, y)),
        (x > 0 && y < height - 1, (x - 1, y + 1)),
        (y > 0, (x, y - 1)),
        (y < height - 1, (x, y + 1)),
        (x < width - 1 && y > 0, (x + 1, y - 1)),
        (x < width - 1, (x + 1, y)),
        (x < width - 1 && y < height - 1, (x + 1, y + 1)),
    ]
    .into_iter()
    .filter_map(|(cond, (dx, dy))| {
        if cond {
            Some((dx as usize, dy as usize))
        } else {
            None
        }
    })
}

// strings
pub fn is_lowercase(s: &str) -> bool {
    s.chars().all(char::is_lowercase)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triangular_test() {
        assert_eq!(triangular(99), 4950);
        assert_eq!(triangular(100), 5050);
    }

    #[test]
    fn mean_test() {
        assert_eq!(mean(&[5, 6, 0, 10, -30]), -1.8);
        assert_eq!(mean(&[0]), 0.0);
    }

    #[test]
    fn median_test() {
        let arr = [10, 11, 13, 15, 16, 23, 26];
        assert_eq!(median(&arr), 15.0);

        let arr = [10, 11, 14, 15];
        assert_eq!(median(&arr), 12.5);
    }

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
