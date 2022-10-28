use std::env;
use std::fs;

pub fn read_file(year: &str, folder: &str, day: u8) -> String {
    let cwd = env::current_dir().unwrap();

    let filepath = cwd
        .join(year)
        .join("src")
        .join(folder)
        .join(format!("day{:02}.txt", day));

    fs::read_to_string(filepath)
        .expect("Could not open input file")
}