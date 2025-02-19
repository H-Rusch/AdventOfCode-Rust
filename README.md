# Advent of Code

This repo contains my solutions for the Advent of Code in Rust.

- [2016](https://adventofcode.com/2016)
- [2021](https://adventofcode.com/2021)
- [2022](https://adventofcode.com/2022)
- [2023](https://adventofcode.com/2023)

## How to run?

To run a specific day of a year execute `cargo run -p <crate_name> -- <day>` where `<crate_name>` is the name of a specific years' crate as specified in the root-`Cargo.toml` such as `aoc22`.

## Automatic input-download

The first time a puzzle is executed the corresponding input is downloaded and cached for subsequent executions (See `util/dlin/download_if_needed()`).

The program expects the `SESSION` variable from the AOC website to be set inside a `.env`-file in order for the request to the website to be successful.

The request to download the puzzle's input contains a `User-Agent` header with the following content:

```txt
https://github.com/H-Rusch/AdventOfCode-Rust contact @ https://github.com/H-Rusch/AdventOfCode-Rust/issues/new
```
