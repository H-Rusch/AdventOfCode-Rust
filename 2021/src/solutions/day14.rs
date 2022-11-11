use std::collections::HashMap;

pub fn part1(input: &str) -> u64 {
    let (mut polymer_map, mut letter_counts, transformations) = parse(input);
    calc_iterations(&mut polymer_map, &mut letter_counts, &transformations, 10)
}

pub fn part2(input: &str) -> u64 {
    let (mut polymer_map, mut letter_counts, transformations) = parse(input);
    calc_iterations(&mut polymer_map, &mut letter_counts, &transformations, 40)
}

fn calc_iterations(
    polymer_map: &mut HashMap<(char, char), u64>,
    letter_counts: &mut HashMap<char, u64>,
    transformations: &HashMap<(char, char), char>,
    num: usize,
) -> u64 {
    for _ in 0..num {
        let mut next_state: HashMap<(char, char), u64> = HashMap::new();

        for (&(c1, c2), n) in polymer_map.iter() {
            let insertion = transformations.get(&(c1, c2)).unwrap();
            *letter_counts.entry(*insertion).or_insert(0) += n;

            let poly1 = (c1, *insertion);
            let poly2 = (*insertion, c2);
            *next_state.entry(poly1).or_insert(0) += n;
            *next_state.entry(poly2).or_insert(0) += n;
        }

        *polymer_map = next_state;
    }

    calc_score(letter_counts)
}

fn calc_score(letter_counts: &HashMap<char, u64>) -> u64 {
    letter_counts.values().max().unwrap() - letter_counts.values().min().unwrap()
}

type ComplexReturn = (
    HashMap<(char, char), u64>,
    HashMap<char, u64>,
    HashMap<(char, char), char>,
);

fn parse(input: &str) -> ComplexReturn {
    let (polymer, transformations) = input.split_once("\n\n").unwrap();

    let polymer_map =
        polymer
            .chars()
            .zip(polymer.chars().skip(1))
            .fold(HashMap::new(), |mut map, (c1, c2)| {
                *map.entry((c1, c2)).or_insert(0) += 1;
                map
            });

    let letter_counts = polymer.chars().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    });

    let transformations = transformations
        .lines()
        .map(|line| line.split_once(" -> ").unwrap())
        .map(|(s1, s2)| (s1.chars(), s2.chars()))
        .map(|(mut ch1, mut ch2)| {
            (
                (ch1.next().unwrap(), ch1.next().unwrap()),
                ch2.next().unwrap(),
            )
        })
        .collect();

    (polymer_map, letter_counts, transformations)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_ex() {
        let input = "\
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

        assert_eq!(part1(input), 1588)
    }

    #[test]
    fn part2_ex() {
        let input = "\
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

        assert_eq!(part2(input), 2188189693529)
    }
}
