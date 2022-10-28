use dotenv::dotenv;
use std::fs;
use std::path::Path;
use reqwest::header::COOKIE;


pub struct Config {
    pub year: String,
    pub day: String,
    pub session_cookie: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let year = match args.next() {
            Some(arg) => arg,
            None => return Err("Must specify year"),
        };
        let day = match args.next() {
            Some(arg) => arg,
            None => return Err("Must specify day"),
        };

        dotenv().ok();
        let session_cookie = match std::env::var("AOC_SESSION_COOKIE") {
            Ok(val) => val,
            Err(_) => return Err("Error reading in session cookie from .env"),
        };

        Ok(Config {year, day, session_cookie})
    }
}

fn make_path_string(config: &Config) -> String {
    format!("./{}/inputs/day{:0>2}.txt", &config.year, &config.day)
}

pub fn run(config: Config) -> Result<&'static str, Box<dyn std::error::Error>> {
    let path_string = make_path_string(&config);
    let path = Path::new(&path_string);
    println!("{:?}", path);

    if path.try_exists()? {
        Ok("File on Path already exists. Skipped download.")
    } else {
        let text = make_request(&config)?;
        fs::write(path, text)?;
        Ok("Wrote content to file.")
    }
}

#[tokio::main]
async fn make_request(config: &Config) -> Result<String, Box<dyn std::error::Error>> {
    let url = vec!["https://adventofcode.com", &config.year, "day", &config.day, "input"]
        .join("/");
    println!("{url}");
        
    let client = reqwest::Client::new();
    let body = client.get(url)
        .header(COOKIE, format!("session={}", &config.session_cookie))
        .send()
        .await?
        .text()
        .await?;

    Ok(body)
}
