use itertools::Itertools;

const LENGTH_1: usize = 272;
const LENGTH_2: usize = 35651584;

pub fn part1(input: &str) -> String {
    generate_checksum_to_fill(input, LENGTH_1)
}

pub fn part2(input: &str) -> String {
    generate_checksum_to_fill(input, LENGTH_2)
}

fn generate_checksum_to_fill(initial_data: &str, limit: usize) -> String {
    let mut data = initial_data.to_string();
    while data.len() < limit {
        data = extend_data(&data);
    }

    generate_checksum(&data, limit)
}

fn extend_data(a: &str) -> String {
    let mut result = String::with_capacity(a.len() * 2 + 1);
    let b = a
        .chars()
        .map(|c| match c {
            '1' => '0',
            '0' => '1',
            _ => unreachable!(),
        })
        .rev()
        .collect::<String>();

    result.push_str(a);
    result.push('0');
    result.push_str(&b);

    result
}

fn generate_checksum(data: &str, limit: usize) -> String {
    let mut result = String::with_capacity(data.len() / 2);
    data.chars().take(limit).tuples().for_each(|(c1, c2)| {
        let ch = match (c1, c2) {
            ('1', '1') | ('0', '0') => '1',
            ('0', '1') | ('1', '0') => '0',
            _ => unreachable!(),
        };
        result.push(ch);
    });

    if result.len() % 2 == 0 {
        result = generate_checksum(&result, result.len());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extend_data_example() {
        assert_eq!("100".to_string(), extend_data("1"));
        assert_eq!("001".to_string(), extend_data("0"));
        assert_eq!("11111000000".to_string(), extend_data("11111"));
        assert_eq!(
            "1111000010100101011110000".to_string(),
            extend_data("111100001010")
        );
    }

    #[test]
    fn generate_checksum_example() {
        assert_eq!("100".to_string(), generate_checksum("110010110100", 12));
    }

    #[test]
    fn generate_checksum_to_fill_example() {
        assert_eq!("01100".to_string(), generate_checksum_to_fill("10000", 20));
    }
}
