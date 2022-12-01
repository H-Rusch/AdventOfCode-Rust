use std::collections::HashMap;

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.split_once(" | ").unwrap().1)
        .map(|output| {
            output
                .split_whitespace()
                .filter(|seg| {
                    let chars = seg.chars().count();
                    chars == 2 || chars == 3 || chars == 4 || chars == 7
                })
                .count()
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let count_to_number = HashMap::from([
        ("467889".to_string(), 0),
        ("89".to_string(), 1),
        ("47788".to_string(), 2),
        ("77889".to_string(), 3),
        ("6789".to_string(), 4),
        ("67789".to_string(), 5),
        ("467789".to_string(), 6),
        ("889".to_string(), 7),
        ("4677889".to_string(), 8),
        ("677889".to_string(), 9),
    ]);

    let list = parse(input);
    list.iter()
        .map(|digits| {
            digits
                .iter()
                .map(|count| count_to_number.get(count).unwrap().to_string())
                .collect::<Vec<String>>()
                .join("")
                .parse::<usize>()
                .unwrap()
        })
        .sum::<usize>()
}

fn parse(input: &str) -> Vec<Vec<String>> {
    input
        .lines()
        .map(|line| {
            let (first, second) = line.split_once(" | ").unwrap();
            let map = count_segments(first);
            convert_to_sorted_counts(&map, second)
        })
        .collect()
}

fn convert_to_sorted_counts(count_map: &HashMap<char, usize>, part: &str) -> Vec<String> {
    part.split_whitespace()
        .map(|pattern| {
            let mut counts: Vec<String> = pattern
                .chars()
                .map(|c| count_map.get(&c).unwrap().to_string())
                .collect();
            counts.sort();
            counts.join("")
        })
        .collect()
}

fn count_segments(first: &str) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for l in first.replace(' ', "").chars() {
        *map.entry(l).or_insert(0) += 1;
    }

    map
}
