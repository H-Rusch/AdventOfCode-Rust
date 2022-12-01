fn convert(line: &str) -> i32 {
    line.parse().unwrap()
}

fn parse(input: &str) -> Vec<i32> {
    input.lines()
        .map(convert)
        .collect()
}

pub fn part1(input: &str) -> i32 {
    let nums = parse(input);
    let mut count = 0;

    for (cur, next) in nums.iter().zip(nums.iter().skip(1)) {
        if next > cur {
            count += 1;
        }
    }

    count
}

pub fn part2(input: &str) -> i32 {
    let nums = parse(input);
    let mut count = 0;

    for (cur, next) in nums.iter().zip(nums.iter().skip(3)) {
        if next > cur {
            count += 1;
        }
    }

    count
}
