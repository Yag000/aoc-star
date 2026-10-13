# aoc-star

[![Crates.io](https://img.shields.io/crates/v/aoc-star.svg)](https://crates.io/crates/aoc-star)
[![Documentation](https://docs.rs/aoc-star/badge.svg)](https://docs.rs/aoc-star)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)

A Rust library and CLI tool to manage your [Advent of Code](https://adventofcode.com/) solutions.

With `aoc-star`, you can annotate your solution functions with a `#[star]` attribute and run them directly from the command line. It also supports fetching puzzle inputs and submitting answers automatically through [`aoc-client`](https://crates.io/crates/aoc-client).

## Installation

Add `aoc-star` to your project:
```bash
cargo add aoc-star
```

If you want to fetch inputs and submit answers directly from Advent of Code, enable the optional `aoc-client` feature:
```bash
cargo add aoc-star --features aoc-client
```

## Quick start

Annotate your solution functions with `#[star]`, specifying the day, part, and year:

```rust
use aoc_star::star;

#[star(day = 1, part = 1, year = 2024)]
fn day1_part1(input: String) -> String {
    input.lines().count().to_string()
}

#[star(day = 1, part = 2, year = 2024)]
fn day1_part2(input: String) -> String {
    input.lines().filter(|line|!line.isempty()).count().to_string() 
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    aoc_star::run()
}

```


Each function receives the puzzle input as a `String` and returns its answer as a `String`. The `#[star]` attribute associates the function with a specific day, part, and year.

Run a solution using a local input file:

```bash
cargo run -- --day 1 --part 1 --year 2024 --input-file path/to/input.txt
```

That's it. `aoc-star` takes care of selecting the right solution and passing it the input.

You can also use the shorter flag names:

```bash
cargo run -- -d 1 -p 1 -y 2024 --input-file path/to/input.txt
```

See the [CLI reference](#cli-reference) for all available options.

## Fetching inputs and submitting answers

If you enabled the `aoc-client` feature and configured your Advent of Code session cookie, you can fetch puzzle inputs directly from Advent of Code without having to download them manually.

For example:
```bash
cargo run -- --day 1 --part 1 --year 2024
```

To submit the computed answer, add `--publish`:

```bash
cargo run -- --day 1 --part 1 --year 2024 --publish
```

Both input fetching and answer submission require the `aoc-client` feature and a valid session cookie.

You can find instructions for configuring your session cookie in the [configuration section](#configuration).

## Example project

For a complete example of `aoc-star` in use, check out the
[AoC Rust solutions repository](https://github.com/Yag000/AoC-rust-solutions).

## Configuration

`aoc-star` supports a configuration file to store your Advent of Code session cookie and a default year.

The year can be specified directly with `--year`. If it isn't provided,
`aoc-star` uses the configured default year, falling back to the current
year when no default is configured.

The configuration is searched for in the following order:

1. `aoc-star.yml` in the current directory.
2. The global configuration file, typically `~/.config/aoc-star/config.yml`.

The local configuration takes precedence over the global configuration.

An example configuration looks like this:

```yaml
token: "your_aoc_session_cookie_here"
year: 2024
```


The `token` field contains your Advent of Code session cookie, while `year` specifies the default year to use when running solutions.

If no configuration file exists, `aoc-star` can create one in the global configuration directory using the `AOC_TOKEN` environment variable and the current year.

You can also use the `--setup` flag to create the configuration file if it doesn't exist.


### Setting up your session cookie

To fetch inputs or submit answers, `aoc-star` needs your Advent of Code session cookie.

To find it, log in to [Advent of Code](https://adventofcode.com/) in your browser and inspect your cookies. Look for the cookie named `session`.

You can either put its value in your configuration file or provide it through the `AOC_TOKEN` environment variable:

```bash
export AOC_TOKEN="your_aoc_session_cookie_here"
```

The environment variable can be used when creating a new configuration file.

Treat your session cookie like a password. Don't commit it to version control or share it publicly.
If you keep your session cookie in a local `aoc-star.yml` file, consider adding that file to your `.gitignore`.

## CLI reference

The runner provided by `aoc_star::run()` accepts the following arguments:

| Flag | Description |
|---|---|
| `-d`, `--day <DAY>` | Advent of Code day (1–25). Required. |
| `-p`, `--part <PART>` | Puzzle part. Defaults to `1`. |
| `-y`, `--year <YEAR>` | Advent of Code year. If omitted, the configured year or current year is used. |
| `--input-file <PATH>` | Read the puzzle input from a local file. Can be omitted if using the `aoc-client` feature to automaticaly fetch the input. |
| `--publish` | Submit the computed answer to Advent of Code. Requires the `aoc-client` feature. |
| `--setup` | Create the configuration file if it doesn't exist, using `AOC_TOKEN` and the current year. |

If a default year is configured, you can omit `--year`:

```bash
cargo run -- --day 1 --part 1
```

Or use the shorter flags:

```bash
cargo run -- -d 1 -p 1
```

When `--input-file` is omitted, the input can be fetched directly from Advent of Code if the `aoc-client` feature is enabled and your session cookie is configured.

For the complete list of available options, run:

```bash
cargo run -- --help
```

## Features

`aoc-star` provides a couple of optional features:

- **`aoc-client`**: Enables fetching puzzle inputs and submitting answers through the [`aoc-client`](https://crates.io/crates/aoc-client) crate.
- **`test-helpers`**: Exposes utilities for testing the runner without relying on actual command-line parsing:
  - `aoc_star::test_helpers::CommandArgument`
  - `aoc_star::test_helpers::run_with_result`

## License

This project is dual-licensed under your choice of either:

- [MIT License](LICENSE-MIT)
- [Apache License, Version 2.0](LICENSE-APACHE)
