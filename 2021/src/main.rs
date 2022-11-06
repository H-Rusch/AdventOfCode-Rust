// general structure taken and adapted from github user fspoettel at https://github.com/fspoettel/advent-of-code-2021/blob/master/src/main.rs

use crate::solutions::*;
use aoc21::read_file;
use std::env;

mod solutions;

macro_rules! solve_day {
    ($day:path, $input:expr) => {{
        use $day::*;
        println!("Part 1: {}", part1($input));
        println!("Part 2: {}", part2($input));
    }};
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: u8 = args[1].clone().parse().unwrap();
    let input = read_file("2021", "inputs", day);

    match day {
        1 => solve_day!(day01, &input),
        2 => solve_day!(day02, &input),
        3 => solve_day!(day03, &input),
        4 => solve_day!(day04, &input),
        5 => solve_day!(day05, &input),
        6 => solve_day!(day06, &input),
        7 => solve_day!(day07, &input),
        8 => solve_day!(day08, &input),
        9 => solve_day!(day09, &input),
        _ => println!("day not solved: {}", day),
    }
}
