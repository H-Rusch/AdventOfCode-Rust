use std::env;
use std::fs;

pub fn read_file(folder: &str, day: u8) -> String {
    let cwd = env::current_dir().unwrap();

    let filepath = cwd
        .join("src")
        .join(folder)
        .join(format!("day{:02}.txt", day));

    fs::read_to_string(filepath)
        .expect("Could not open input file")
}