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
}