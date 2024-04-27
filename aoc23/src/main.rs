// general structure taken and adapted from github user fspoettel at https://github.com/fspoettel/advent-of-code-2021/blob/master/src/main.rs

use std::{time::SystemTime, process};

use crate::solutions::*;
use std::env;

mod solutions;

macro_rules! solve_day {
    ($day:path, $input:expr) => {{
        use $day::*;

        let start = SystemTime::now();
        let res1 = part1($input);
        let time1 = start.elapsed().unwrap();
        println!("Part 1: {}\n {:?}", res1, time1);

        let start = SystemTime::now();
        let res2 = part2($input);
        let time2 = start.elapsed().unwrap();
        println!("Part 2: {}\n {:?}", res2, time2);
    }};
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: u8 = args[1].clone().parse().unwrap();

    let input = match util::dlin::read_input("aoc23", "2023", day) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            process::exit(1);
        },
    };

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
        10 => solve_day!(day10, &input),
        11 => solve_day!(day11, &input),
        12 => solve_day!(day12, &input),
        13 => solve_day!(day13, &input),
        14 => solve_day!(day14, &input),
        15 => solve_day!(day15, &input),
        16 => solve_day!(day16, &input),
        17 => solve_day!(day17, &input),
        18 => solve_day!(day18, &input),
        19 => solve_day!(day19, &input),
        _ => println!("day not solved: {}", day),
    }
}
