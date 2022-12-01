use std::collections::HashMap;

pub fn part1(input: &str) -> u32 {
    let (mut p1, mut p2) = parse(input);
    let mut scores = (0, 0);

    let mut dice = (1..=100).cycle();
    let mut roll_count = 0;

    loop {
        let rolled = roll_dice(&mut dice);
        roll_count += 3;
        
        p1 += rolled;
        p1 = (p1 % 10) + 10 * (p1 % 10 == 0) as u32;
        scores.0 += p1;

        if scores.0 >= 1000 {
            return scores.1 * roll_count;
        }


        let rolled = roll_dice(&mut dice);
        roll_count += 3;
        
        p2 += rolled;
        p2 = (p2 % 10) + 10 * (p2 % 10 == 0) as u32;
        scores.1 += p2;

        if scores.1 >= 1000 {
            return scores.0 * roll_count;
        }
    }
}

fn roll_dice(dice: &mut dyn Iterator<Item = u32>) -> u32 {
    dice.take(3).sum()
}

pub fn part2(input: &str) -> u64 {
    let mut cache: HashMap<(u64, u64, u64, u64), (u64, u64)> = HashMap::new();
    let (p1, p2) = parse(input);

    let (p1_wins, p2_wins) = play_quantum(p1 as u64, p2 as u64, 0, 0, &mut cache);
    p1_wins.max(p2_wins)
}

fn play_quantum(current_pos: u64, other_pos: u64, current_score: u64, other_score: u64, cache: &mut HashMap<(u64, u64, u64, u64), (u64, u64)>) -> (u64, u64) {
    if current_score >= 21 {
        return (1, 0);
    } 
    if other_score >= 21 {
        return (0, 1);
    }
    if cache.contains_key(&(current_pos, other_pos, current_score, other_score)) {
        return *cache.get(&(current_pos, other_pos, current_score, other_score)).unwrap();
    }

    let mut result = (0, 0);
    for (rolled, times) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
        let mut new_pos = current_pos + rolled;
        new_pos = (new_pos % 10) + 10 * (new_pos % 10 == 0) as u64;

        let (other_wins, current_wins) = play_quantum(other_pos, new_pos, other_score, current_score + new_pos, cache);
        result = (result.0 + times * current_wins, result.1 + times * other_wins);
    }
    cache.insert((current_pos, other_pos, current_score, other_score), result);

    result
}

fn parse(input: &str) -> (u32, u32) {
    let (p1, p2) = input.split_once('\n').unwrap();
    let p1 = p1.split(' ').collect::<Vec<&str>>()[4].parse().unwrap();
    let p2 = p2.split_whitespace().collect::<Vec<&str>>()[4].parse().unwrap();
    
    (p1, p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex() {
        let input = "`\
Player 1 starting position: 4
Player 2 starting position: 8
";

        assert_eq!(part1(input), 739785);
    }

    #[test]
    fn part2_ex() {
        let input = "`\
Player 1 starting position: 4
Player 2 starting position: 8
";

        assert_eq!(part2(input), 444356092776315);
    }
}