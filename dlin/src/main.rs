use std::env;
use std::process;

use dlin::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem setting up configuration: {}", err);
        process::exit(1);
    });

    match dlin::run(config) {
        Ok(s) => println!("{s}"),
        Err(e) => {
            eprintln!("An Error occurred: {e}");
            process::exit(1);
        },
    }
}

