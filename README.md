# Advent of Code
Solutions for the Advent of Code on my journey of learning the Rust programming language.

To run a day in a specific year execute `cargo run -p <crate_name> -- <day>` where `<crate_name>` is the name of a specific years' crate as specified in the root-`Cargo.toml`.

## Automated download of inputs
A puzzle's input is downloaded the first time a day is executed and then cached after the initial download (`util/dlin/download_if_needed()`). 

In case the input is corrupted, delete the file manually and run the code again, to download a fresh copy of the input.

`User-Agent` header: 
```
https://github.com/H-Rusch/AdventOfCode-Rust contact @ https://github.com/H-Rusch/AdventOfCode-Rust/issues/new
```


## Solutions for years
- [2016](https://adventofcode.com/2016)
- [2021](https://adventofcode.com/2021)
- [2022](https://adventofcode.com/2022)
- [2023](https://adventofcode.com/2023)