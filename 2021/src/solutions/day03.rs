pub fn part1(input: &str) -> u32 {
    let numbers = parse(input);

    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;
    for i in 0..numbers[0].len() {
        let b = most_common_at(&numbers, i);
        gamma <<= 1;
        gamma += b as u32;

        epsilon <<= 1;
        epsilon += (b as u32 + 1) % 2;
    }

    gamma * epsilon
}

pub fn part2(input: &str) -> u32 {
    let numbers = parse(input);

    let oxygen = find_rating(numbers.clone(), false);
    let co2 = find_rating(numbers, true);
    
    oxygen * co2
}

fn find_rating(mut list: Vec<Vec<u8>>, co2: bool) -> u32 {
    let mut pos = 0;
    while list.len() > 1 {
        // most common bit for oxygen and least common bit for co2
        let bit = if co2 { least_common_at(&list, pos) } else { most_common_at(&list, pos) };   

        list.retain(|v| v[pos] == bit);
        
        pos += 1;
    }

    convert_to_int(&list[0])
}

fn convert_to_int(v: &Vec<u8>) -> u32 {
    let mut num = 0;
    for i in v {
        num <<= 1;
        num += *i as u32;
    }

    num
}

fn least_common_at(list: &Vec<Vec<u8>>, pos: usize) -> u8 {
    (most_common_at(list, pos) + 1) % 2
}

fn most_common_at(list: &Vec<Vec<u8>>, pos: usize) -> u8 {
    let count = list.iter()
        .map(|l| l[pos])
        .filter(|&b| b == 1)
        .count() as f64;

    if count >= list.len() as f64 / 2.0 {
        1
    } else {
        0
    }
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    input.lines()
        .map(|l| l.chars()
            .map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect()
}
