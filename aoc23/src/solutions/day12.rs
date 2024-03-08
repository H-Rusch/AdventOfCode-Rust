use itertools::Itertools;

pub fn part1(input: &str) -> u64 {
    let conditional_records = parse(input);
    conditional_records
        .into_iter()
        .map(|(row, rules)| count_arrangements(row, rules))
        .sum()
}

pub fn part2(_input: &str) -> u64 {
    0
}

fn count_arrangements(springs: &str, damaged_groups: Vec<usize>) -> u64 {
    count_recursive(&springs.chars().collect_vec(), &damaged_groups)
}

fn count_recursive(springs: &[char], damaged_groups: &[usize]) -> u64 {
    if springs.is_empty() {
        return if damaged_groups.is_empty() { 1 } else { 0 };
    }

    match springs[0] {
        '.' => count_recursive(&springs[1..], damaged_groups),
        '#' => {
            if damaged_groups.is_empty() {
                return 0;
            }

            let group_size = damaged_groups[0];
            if group_size > springs.len()
                || springs.iter().take(group_size).any(|&ch| ch == '.')
                || springs.iter().skip(group_size).take(1).any(|&ch| ch == '#')
            {
                return 0;
            }

            let new_groups = damaged_groups[1..].to_owned();
            if group_size == springs.len() {
                return if new_groups.is_empty() { 1 } else { 0 };
            }

            count_recursive(&springs[(group_size + 1)..], &new_groups)
        }
        '?' => ['#', '.']
            .iter()
            .map(|ch| {
                let mut replacement = springs.to_owned();
                replacement[0] = *ch;
                replacement
            })
            .map(|next_row| count_recursive(&next_row, damaged_groups))
            .sum(),
        _ => unreachable!(),
    }
}

fn parse(input: &str) -> Vec<(&str, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let (row, rules) = line.split_once(' ').unwrap();
            let rules = rules.split(',').map(|n| n.parse().unwrap()).collect();
            (row, rules)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../examples/day12.txt");

    #[test]
    fn count_arrangements_test() {
        assert_eq!(1, count_arrangements("???.###", vec![1, 1, 3]));
        assert_eq!(4, count_arrangements(".??..??...?##.", vec![1, 1, 3]));
        assert_eq!(1, count_arrangements("?#?#?#?#?#?#?#?", vec![1, 3, 1, 6]));
        assert_eq!(1, count_arrangements("????.#...#...", vec![4, 1, 1]));
        assert_eq!(4, count_arrangements("????.######..#####.", vec![1, 6, 5]));
        assert_eq!(10, count_arrangements("?###????????", vec![3, 2, 1]));
    }

    #[test]
    fn part1_ex() {
        assert_eq!(21, part1(EXAMPLE));
    }

    #[test]
    fn part2_ex() {}
}
