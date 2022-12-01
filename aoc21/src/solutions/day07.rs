pub fn part1(input: &str) -> i32 {
    let nums = parse(input);
    let median = util::median(&nums[..]) as i32;

    nums.iter().map(|n| (n - median).abs()).sum()
}

pub fn part2(input: &str) -> i32 {
    let mut nums = parse(input);
    nums.sort();

    let range = nums[0]..nums[nums.len() - 1];
    range.map(|i| {
        nums.iter().map(|n| util::triangular(n.abs_diff(i)))
            .sum::<u32>()
    }).min().unwrap() as i32
}

fn parse(input: &str) -> Vec<i32> {
    input
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex() {
        let input = "16,1,2,0,4,2,7,1,2,14";

        assert_eq!(part1(input), 37);
    }

    #[test]
    fn part2_ex() {
        let input = "16,1,2,0,4,2,7,1,2,14";

        assert_eq!(part2(input), 168);
    }
}
