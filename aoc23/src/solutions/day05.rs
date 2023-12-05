use rayon::prelude::*;

struct Translation {
    source_start: usize,
    destination_start: usize,
    range_length: usize,
}

impl Translation {
    fn contains(&self, value: &usize) -> bool {
        let range = self.source_start..(self.source_start + self.range_length);
        range.contains(value)
    }

    fn translate(&self, value: usize) -> usize {
        let offset = value - self.source_start;
        self.destination_start + offset
    }
}

pub fn part1(input: &str) -> usize {
    let (mut seeds, translations) = parse(input);

    translate(&mut seeds, &translations);

    *seeds.iter().min().unwrap()
}

pub fn part2(input: &str) -> usize {
    let (mut seeds, translations) = parse(input);
    seeds = expand_seeds(&seeds);

    translate(&mut seeds, &translations);

    *seeds.iter().min().unwrap()
}

fn translate(values: &mut Vec<usize>, translations: &[Vec<Translation>]) {
    for section in translations.iter() {
        values
            .par_iter_mut()
            .for_each(|value| perform_section_translations(value, section));
    }
}

fn perform_section_translations(value: &mut usize, translations: &[Translation]) {
    for translation in translations {
        if translation.contains(value) {
            *value = translation.translate(*value);
            return;
        }
    }
}

fn expand_seeds(seeds: &[usize]) -> Vec<usize> {
    let mut new_seeds = Vec::new();
    seeds
        .chunks(2)
        .for_each(|chunk| new_seeds.extend(chunk[0]..(chunk[0] + chunk[1])));

    new_seeds
}

fn parse(input: &str) -> (Vec<usize>, Vec<Vec<Translation>>) {
    let mut parts = input.split("\n\n");
    let seeds = parse_seeds(parts.next().unwrap());
    let translations = parts.map(parse_mapping).collect();

    (seeds, translations)
}

fn parse_seeds(line: &str) -> Vec<usize> {
    line.strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn parse_mapping(part: &str) -> Vec<Translation> {
    part.lines()
        .skip(1)
        .map(|line| {
            let values: Vec<_> = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            Translation {
                range_length: values[2],
                source_start: values[1],
                destination_start: values[0],
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../examples/day05.txt");

    #[test]
    fn translation_test() {
        let translation = Translation {
            source_start: 50,
            destination_start: 52,
            range_length: 48,
        };

        assert_eq!(53, translation.translate(51));
    }

    #[test]
    fn part1_ex() {
        assert_eq!(35, part1(EXAMPLE));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(46, part2(EXAMPLE));
    }
}
