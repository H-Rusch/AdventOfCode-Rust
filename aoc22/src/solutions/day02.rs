use RpsResult::*;
use RPS::*;

// Rock-Paper-Scissors
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn from(letter: &str) -> RPS {
        match letter {
            "A" | "X" => RPS::Rock,
            "B" | "Y" => RPS::Paper,
            "C" | "Z" => RPS::Scissors,
            _ => unreachable!(),
        }
    }

    fn play(choice: &RPS, opponent: &RPS) -> RpsResult {
        match (choice, opponent) {
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => Draw,
            (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => Loss,
            (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => Win,
        }
    }

    fn score(&self) -> usize {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    // get the Rock-Paper-Scisscors selection the player has to chose in order to get the desired result
    fn from_result(opponent: &RPS, result: &RpsResult) -> RPS {
        match (opponent, result) {
            (Rock, Win) | (Paper, Draw) | (Scissors, Loss) => Paper,
            (Scissors, Win) | (Rock, Draw) | (Paper, Loss) => Rock,
            (Paper, Win) | (Scissors, Draw) | (Rock, Loss) => Scissors,
        }
    }
}

enum RpsResult {
    Win,
    Loss,
    Draw,
}

impl RpsResult {
    fn from(letter: &str) -> RpsResult {
        match letter {
            "X" => Self::Loss,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => unreachable!(),
        }
    }

    fn score(&self) -> usize {
        match self {
            RpsResult::Loss => 0,
            RpsResult::Draw => 3,
            RpsResult::Win => 6,
        }
    }
}

pub fn part1(input: &str) -> usize {
    let strategy_guide = parse(input);

    strategy_guide
        .iter()
        .map(|&(opponent, choice)| {
            let (opponent, choice) = (RPS::from(opponent), RPS::from(choice));

            choice.score() + RPS::play(&choice, &opponent).score()
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let strategy_guide = parse(input);

    strategy_guide
        .iter()
        .map(|&(opponent, result)| {
            let (opponent, result) = (RPS::from(opponent), RpsResult::from(result));
            
            result.score() + RPS::from_result(&opponent, &result).score()
        })
        .sum()
}

fn parse(input: &str) -> Vec<(&str, &str)> {
    input
        .trim()
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
A Y
B X
C Z";

    #[test]
    fn part1_ex() {
        assert_eq!(15, part1(INPUT));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(12, part2(INPUT));
    }
}
