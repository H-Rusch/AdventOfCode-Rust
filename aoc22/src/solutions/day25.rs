pub fn part1(input: &str) -> String {
    let nums = parse(input);

    to_snafu(nums.iter().sum())
}

pub fn part2(_: &str) -> &'static str {
    "All done :)"
}

fn to_snafu(num: i64) -> String {
    let mut num = num;
    let mut result = String::new();

    while num > 0 {
        match num % 5 {
            0 => result.push('0'),
            1 => result.push('1'),
            2 => result.push('2'),
            3 => {
                result.push('=');
                num += 5;
            }
            4 => {
                result.push('-');
                num += 5;
            }
            _ => unreachable!(),
        }

        num /= 5;
    }

    result.chars().rev().collect()
}

const FACTOR: i64 = 5;

fn to_int(snafu: &str) -> i64 {
    let mut num = 0;
    for (power, c) in snafu.chars().rev().enumerate() {
        num += match c {
            '=' => -2 * FACTOR.pow(power as u32),
            '-' => -(FACTOR).pow(power as u32),
            '0' => 0,
            '1' => FACTOR.pow(power as u32),
            '2' => 2 * FACTOR.pow(power as u32),
            _ => unreachable!(),
        }
    }
    num
}

fn parse(input: &str) -> Vec<i64> {
    input.lines().map(to_int).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../examples/day25.txt");

    #[test]
    fn convert_from_snafu() {
        assert_eq!(to_int("1=-0-2"), 1747);
        assert_eq!(to_int("2=0="), 198);
        assert_eq!(to_int("21"), 11);
        assert_eq!(to_int("2=01"), 201);
        assert_eq!(to_int("111"), 31);
        assert_eq!(to_int("20012"), 1257);
        assert_eq!(to_int("112"), 32);
        assert_eq!(to_int("1=-1="), 353);
        assert_eq!(to_int("1-12"), 107);
        assert_eq!(to_int("12"), 7);
        assert_eq!(to_int("1="), 3);
        assert_eq!(to_int("122"), 37);
    }

    #[test]
    fn convert_to_snafu() {
        assert_eq!("1=-0-2".to_string(), to_snafu(1747));
        assert_eq!("2=0=".to_string(), to_snafu(198));
        assert_eq!("21".to_string(), to_snafu(11));
        assert_eq!("2=01".to_string(), to_snafu(201));
        assert_eq!("111".to_string(), to_snafu(31));
        assert_eq!("20012".to_string(), to_snafu(1257));
        assert_eq!("112".to_string(), to_snafu(32));
        assert_eq!("1=-1=".to_string(), to_snafu(353));
        assert_eq!("1-12".to_string(), to_snafu(107));
        assert_eq!("12".to_string(), to_snafu(7));
        assert_eq!("1=".to_string(), to_snafu(3));
        assert_eq!("122".to_string(), to_snafu(37));
    }

    #[test]
    fn part1_ex() {
        assert_eq!("2=-1=0", part1(INPUT));
    }
}
