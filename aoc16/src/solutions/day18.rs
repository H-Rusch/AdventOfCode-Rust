use std::collections::HashMap;

pub fn part1(input: &str) -> usize {
    let mut traps = parse(input);
    build_grid(&mut traps, 40, input.len());
    count_safe(&traps)
}

pub fn part2(input: &str) -> usize {
    let mut traps = parse(input);
    build_grid(&mut traps, 400_000, input.len());
    count_safe(&traps)
}

fn build_grid(traps: &mut HashMap<(i32, i32), bool>, rows_to_add: usize, width: usize) {
    for y in 1..rows_to_add {
        build_row(traps, y as i32, width);
    }
}

fn build_row(traps: &mut HashMap<(i32, i32), bool>, y: i32, width: usize) {
    for x in 0..width {
        let coordinate = (x as i32, y);
        let trap = is_trap(traps, &coordinate);
        traps.insert(coordinate, trap);
    }
}

fn is_trap(traps: &HashMap<(i32, i32), bool>, coordinate: &(i32, i32)) -> bool {
    let (x, y) = *coordinate;
    let left = *traps.get(&(x - 1, y - 1)).unwrap_or(&false);
    let center = *traps.get(&(x, y - 1)).unwrap_or(&false);
    let right = *traps.get(&(x + 1, y - 1)).unwrap_or(&false);

    left != center && center == right || left == center && center != right
}

fn count_safe(traps: &HashMap<(i32, i32), bool>) -> usize {
    traps.values().filter(|b| !**b).count()
}

fn parse(input: &str) -> HashMap<(i32, i32), bool> {
    input
        .char_indices()
        .map(|(x, ch)| ((x as i32, 0), ch == '^'))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ".^^.^.^^^^";

    #[test]
    fn part1_ex() {
        let mut traps = parse(INPUT);
        build_grid(&mut traps, 10, INPUT.len());
        count_safe(&traps);
        assert_eq!(38, count_safe(&traps));
    }
}
