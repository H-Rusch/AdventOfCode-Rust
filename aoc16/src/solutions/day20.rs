pub fn part1(input: &str) -> u32 {
    let ranges = parse(input);
    find_lowest_matching(&ranges).unwrap()
}

pub fn part2(input: &str) -> usize {
    let ranges = parse(input);
    count_allowed(&ranges)
}

fn find_lowest_matching(ranges: &[(u32, u32)]) -> Option<u32> {
    let mut last_high = 0;

    for &(low, high) in ranges {
        if last_high < low {
            return Some(last_high);
        }
        // panics if high == u32::MAX. But this does not happen in my case and therefore I wont handle it
        last_high = last_high.max(high + 1);
    }

    None
}

fn count_allowed(ranges: &[(u32, u32)]) -> usize {
    let mut allowed_count = 0;
    let mut last_high = 0;

    for &(low, high) in ranges {
        if last_high < low {
            allowed_count += (low - last_high) as usize;
        }

        last_high = if high == u32::MAX {
            u32::MAX
        } else {
            last_high.max(high + 1)
        }
    }

    allowed_count + (u32::MAX - last_high) as usize
}

fn parse(input: &str) -> Vec<(u32, u32)> {
    let mut ranges: Vec<(u32, u32)> = input
        .lines()
        .map(|line| {
            let (low, high) = line.split_once('-').unwrap();
            (low.parse().unwrap(), high.parse().unwrap())
        })
        .collect();
    ranges.sort();

    ranges
}
