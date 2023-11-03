use dotenv::dotenv;
use reqwest::header::COOKIE;
use std::env;
use std::error;
use std::fs;
use std::path::PathBuf;

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
            Err(_) => return Err("Error reading session cookie from .env. Entry 'session' not found."),
        };

        Ok(Config {
            year,
            day,
            session_cookie,
        })
    }
}

pub fn read_input(
    aoc_folder: &str,
    input_folder: &str,
    year: &str,
    day: u8,
) -> Result<String, Box<dyn error::Error>> {
    let day = &day.to_string();

    let config = Config::build(year, day)?;

    let path = make_path_buf(aoc_folder, input_folder, day);

    download_if_needed(&path, &config)
}

fn make_path_buf(aoc_folder: &str, input_folder: &str, day: &str) -> PathBuf {
    env::current_dir()
        .unwrap()
        .join(aoc_folder)
        .join(input_folder)
        .join(format!("day{:0>2}.txt", day))
}

fn download_if_needed(path: &PathBuf, config: &Config) -> Result<String, Box<dyn error::Error>> {
    if path.try_exists()? {
        // read content of cached file
        match fs::read_to_string(path) {
            Ok(t) => Ok(t),
            Err(e) => Err(e.into()),
        }
    } else {
        // download input file and return content
        let text = make_request(config)?.trim().to_string();

        if text.starts_with("Puzzle inputs differ by user.  Please log in to get your puzzle input.") {
            Err("User is currenty not logged in.".into())
        } else {
            fs::write(path, &text)?;
            Ok(text)
        }
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
