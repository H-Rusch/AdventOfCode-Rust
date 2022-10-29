fn split_to_tuple(line: &str) -> (&str, i32) {
    let (inst, num) = line.split_once(' ').unwrap();

    (inst, num.parse().unwrap())
}

fn parse(input: &str) -> Vec<(&str, i32)> {
    input.lines()
        .map(split_to_tuple)
        .collect()
}

pub fn part1(input: &str) -> i32 {
    let mut x = 0;
    let mut y = 0;

    for (inst, num) in parse(input) {
        match inst {
            "forward" => x += num,
            "up" => y -= num,
            "down" => y += num,
            _ => panic!("unknown instruction {}", inst),
        };
    }

    x * y
}

pub fn part2(input: &str) -> i32 {
    let mut aim = 0;
    let mut x = 0;
    let mut y = 0;

    for (inst, num) in parse(input) {
        match inst {
            "forward" => {
                x += num;
                y += aim * num;
            }
            "up" => aim -= num,
            "down" => aim += num,
            _ => panic!("Unknown instruction {}", inst),
        };
    }

    x * y
}
