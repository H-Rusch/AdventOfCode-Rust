use regex::Regex;

struct Disc {
    positions: usize,
    start_position: usize,
}

pub fn part1(input: &str) -> usize {
    let discs = parse(input);
    find_winning_time(&discs)
}

pub fn part2(input: &str) -> usize {
    let mut discs = parse(input);
    discs.push(Disc { positions: 11, start_position: 0 });
    find_winning_time(&discs)
}

fn find_winning_time(discs: &[Disc]) -> usize {
    let mut time = 0;
    loop {
        if wins_capsule(discs, time) {
            return time;
        }
        time += 1;
    }
}

fn wins_capsule(discs: &[Disc], time: usize) -> bool {
    discs
        .iter()
        .enumerate()
        .all(|(dt, disc)| (disc.start_position + time + dt + 1) % disc.positions == 0)
}

fn parse(input: &str) -> Vec<Disc> {
    let disc_regex =
        Regex::new(r"Disc #\d+ has (\d+) positions; at time=0, it is at position (\d+).").unwrap();
    input
        .lines()
        .map(move |line| {
            let captures = disc_regex.captures(line).unwrap();
            Disc {
                positions: captures.get(1).unwrap().as_str().parse().unwrap(),
                start_position: captures.get(2).unwrap().as_str().parse().unwrap(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../examples/day15.txt");

    #[test]
    fn part1_ex() {
        assert_eq!(5, part1(INPUT));
    }
}
