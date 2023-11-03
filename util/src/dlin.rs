use dotenv::dotenv;
use reqwest::header::COOKIE;
use std::{env, error, io, fs};
use std::path::PathBuf;

const INPUT_FOLDER: &str = "inputs";

struct Config {
    pub year: String,
    pub day: String,
    pub session_cookie: String,
}

impl Config {
    pub fn build(year: &str, day: &str) -> Result<Config, &'static str> {
        let year = year.to_string();
        let day = day.to_string();

        dotenv().ok();
        let session_cookie = match std::env::var("session") {
            Ok(val) => val,
            Err(_) => {
                return Err("Error reading session cookie from .env. Entry 'session' not found.")
            }
        };

        Ok(Config {
            year,
            day,
            session_cookie,
        })
    }
}

pub fn read_input(
    aoc_directory: &str,
    year: &str,
    day: u8,
) -> Result<String, Box<dyn error::Error>> {
    let day = &day.to_string();
    let config = Config::build(year, day)?;

    let path = build_inputs_path(aoc_directory);
    create_inputs_dir(&path)?;
    let path = build_day_input_path(path, day);
    download_if_needed(&path, &config)?;
    read_cached_file(&path)
}

fn build_inputs_path(aoc_directory: &str) -> PathBuf {
    env::current_dir()
        .unwrap()
        .join(aoc_directory)
        .join(INPUT_FOLDER)
}

fn create_inputs_dir(path: &PathBuf) -> Result<(), io::Error> {
    if !path.try_exists()? {
        fs::create_dir(path)?;
    }

    Ok(())
}

fn build_day_input_path(path: PathBuf, day: &str) -> PathBuf {
    path.join(format!("day{:0>2}.txt", day))
}

fn download_if_needed(path: &PathBuf, config: &Config) -> Result<(), Box<dyn error::Error>> {
    if !path.try_exists()? {
        let text = make_request(config)?.trim().to_string();

        if text.starts_with("Puzzle inputs differ by user.") {
            return Err("User is currenty not logged in.".into());
        }
        fs::write(path, &text)?;
    }

    Ok(())
}

fn read_cached_file(path: &PathBuf) -> Result<String, Box<dyn error::Error>> {
    match fs::read_to_string(path) {
        Ok(text) => Ok(text),
        Err(e) => Err(e.into()),
    }
}

#[tokio::main]
async fn make_request(config: &Config) -> Result<String, Box<dyn error::Error>> {
    let url = [
        "https://adventofcode.com",
        &config.year,
        "day",
        &config.day,
        "input",
    ]
    .join("/");

    let client = reqwest::Client::new();
    let body = client
        .get(url)
        .header(COOKIE, format!("session={}", &config.session_cookie))
        .send()
        .await?
        .text()
        .await?;

    Ok(body)
}
