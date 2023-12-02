use lazy_static::lazy_static;
use regex::Regex;

struct Game {
    id: usize,
    measurements: Vec<Measurement>,
}

impl Game {
    fn new(id: usize, measurements: Vec<Measurement>) -> Self {
        Game { id, measurements }
    }

    fn from(line: &str) -> Self {
        let (id, cube_subsets) = line.split_once(": ").unwrap();
        let id = id.strip_prefix("Game ").unwrap().parse().unwrap();
        let measurements = cube_subsets
            .split(';')
            .map(Measurement::get_from_counts)
            .collect();

        Game::new(id, measurements)
    }

    fn is_possible(&self) -> bool {
        self.measurements.iter().all(Measurement::is_possible)
    }

    fn minimum_cubes(&self) -> Measurement {
        let max_red = self.measurements.iter().max_by_key(|m| m.red).unwrap().red;
        let max_green = self.measurements.iter().max_by_key(|m| m.green).unwrap().green;
        let max_blue = self.measurements.iter().max_by_key(|m| m.blue).unwrap().blue;

        Measurement::new(max_red, max_green, max_blue)
    }
}

struct Measurement {
    red: usize,
    green: usize,
    blue: usize,
}

impl Measurement {
    fn new(red: usize, green: usize, blue: usize) -> Self {
        Measurement { red, green, blue }
    }

    fn get_from_counts(color_counts: &str) -> Self {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        COLORS.captures_iter(color_counts).for_each(|capture| {
            let amount = capture.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let color = capture.get(2).unwrap().as_str();

            match color {
                "red" => red = amount,
                "green" => green = amount,
                "blue" => blue = amount,
                _ => unreachable!(),
            }
        });

        Measurement::new(red, green, blue)
    }

    fn is_possible(&self) -> bool {
        self.red <= RED_COUNT && self.green <= GREEN_COUNT && self.blue <= BLUE_COUNT
    }

    fn calc_power(self) -> usize {
        self.red * self.green * self.blue
    }
}

const RED_COUNT: usize = 12;
const GREEN_COUNT: usize = 13;
const BLUE_COUNT: usize = 14;

lazy_static! {
    static ref COLORS: Regex = Regex::new(r"(\d+) (red|green|blue)").unwrap();
}

pub fn part1(input: &str) -> usize {
    let games = parse(input);

    games
        .iter()
        .filter(|game| game.is_possible())
        .map(|game| game.id)
        .sum()
}

pub fn part2(input: &str) -> usize {
    let games = parse(input);

    games
        .iter()
        .map(Game::minimum_cubes)
        .map(Measurement::calc_power)
        .sum()
}

fn parse(input: &str) -> Vec<Game> {
    input.lines().map(Game::from).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../examples/day02.txt");

    #[test]
    fn parse_game_correctly() {
        let game =
            Game::from("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red");

        assert_eq!(3, game.id);
        assert_eq!(3, game.measurements.len());
        assert_eq!(8, game.measurements[0].green);
        assert_eq!(6, game.measurements[0].blue);
        assert_eq!(20, game.measurements[0].red);
    }

    #[test]
    fn part1_ex() {
        assert_eq!(8, part1(EXAMPLE));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(2286, part2(EXAMPLE));
    }
}
