pub mod dlin;
pub mod grid;

use num_traits::PrimInt;

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

pub fn manhatten_distance<T: PrimInt + num_traits::Signed>(x1: T, y1: T, x2: T, y2: T) -> T {
    (x1 - x2).abs() + (y1 - y2).abs()
}


// strings
pub fn is_lowercase(s: &str) -> bool {
    s.chars().all(char::is_lowercase)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manhatten_distance_test() {
        assert_eq!(26, manhatten_distance(2, 3, 15, 16));
        assert_eq!(3378415, manhatten_distance(-1242, 2488142, 888815, -216));
    }

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
}
