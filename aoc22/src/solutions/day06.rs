use std::collections::HashSet;

pub fn part1(input: &str) -> usize {
    find_start(input, 4).unwrap()
}

pub fn part2(input: &str) -> usize {
    find_start(input, 14).unwrap()
}

fn find_start(input: &str, window_width: usize) -> Option<usize> {
    (window_width..input.len()).find(|&i| check_unique(&input[i - window_width..i]))
}

fn check_unique(slice: &str) -> bool {
    slice.chars().collect::<HashSet<char>>().len() == slice.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const INPUT2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const INPUT3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const INPUT4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const INPUT5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn part1_ex() {
        assert_eq!(7, part1(INPUT1));
        assert_eq!(5, part1(INPUT2));
        assert_eq!(6, part1(INPUT3));
        assert_eq!(10, part1(INPUT4));
        assert_eq!(11, part1(INPUT5));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(19, part2(INPUT1));
        assert_eq!(23, part2(INPUT2));
        assert_eq!(23, part2(INPUT3));
        assert_eq!(29, part2(INPUT4));
        assert_eq!(26, part2(INPUT5));
    }
}
