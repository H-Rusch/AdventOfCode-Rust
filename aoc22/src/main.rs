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

    let input = match util::dlin::read_input("aoc22", "inputs", "2022", day) {
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
        _ => println!("day not solved: {}", day),
    }
}
