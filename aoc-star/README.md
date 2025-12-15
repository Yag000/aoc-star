# aoc-star

Library and CLI tool to manage your [Advent of Code](https://adventofcode.com/)
solutions.

This crate:

- lets you annotate solution functions with a `#[star(day = ..., part = ..., year = ...)]` attribute
- and provides a small CLI runner that picks the right solution based on command
  line flags, loads inputs and (optionally) submits answers via
  [`aoc-client`](https://crates.io/crates/aoc-client).

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
aoc-star = "0.1"
```

Enable the optional features as needed:

```toml
[dependencies]
aoc-star = { version = "0.1", features = ["aoc-client"] }
```

They allow you to automatically fetch puzzle inputs and submit answers to Advent of Code provided
you configure your session cookie (see below).

## Usage

Add the dependency to your `Cargo.toml`, by running:

```sh
cargo add aoc-star
```

Annotate your solution functions with the `#[star(...)]` attribute, specifying
the day, part, and year:

```rust
use aoc_star::star;

#[star(day = 1, part = 1, year = 2024)]
fn day1_part1(input: String) -> String {
    // implement your solution using the full puzzle input in `input`
    input.lines().count().to_string()
}

#[star(day = 1, part = 2, year = 2024)]
fn day1_part2(input: String) -> String {
    // ...
    "42".to_string()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    aoc_star::run()
}
```

Build and run:

```sh
cargo run -- --day 1 --part 1 --year 2024 --input-file path/to/input.txt
```

If you enable the `aoc-client` feature and configure your session cookie
(see below), you can omit `--input-file` and the input will be fetched from
Advent of Code directly. You can also use `--publish` to submit your answer:

```sh
cargo run -- --day 1 --part 1 --year 2024 --publish
```

Since a single year is used in all functions, you can set it in the config file (see below) and
omit `--year`:

``` sh
cargo run -- --day 1 --part 1
```

You can also use a less verbose syntax and use single-dash flags:

```sh
cargo run -- -d 1 -p 1
```

See the [CLI flags](#cli-flags) section for more details and run `cargo run -- --help`
to see all available options.

### Example

You can find a complete example project [here](https://github.com/Yag000/AoC-rust-solutions).

## CLI flags

The runner provided by `aoc-star::run()` accepts:

- `-d`, `--day <DAY>`: Advent of Code day (1–25). **Required.**
- `-p`, `--part <PART>`: puzzle part (usually `1` or `2`, defaults to `1`).
- `-y`, `--year <YEAR>`: Advent of Code year. Optional; when omitted, it is
  resolved from config or the current year.
- `--input-file <PATH>`: read the puzzle input from `PATH`. If omitted and
  `aoc-client` is enabled, the input will be fetched remotely.
- `--publish`: when `aoc-client` is enabled, submit the computed answer to
  Advent of Code and show the outcome.
- `--setup`: If the config file does not exist, create it using the value of
  the `AOC_TOKEN` environment variable as the session cookie and the current year.

## Configuration

`aoc-star` can read a config file to determine:

- your session cookie (needed for remote input fetching and answer submission),
- a default year.

The config is searched in:

1. the current directory (`aoc-star.yml`), then
2. the global config directory (typically `~/.config/aoc-star/config.yml`).

If none exists, a new config file is created in the global config directory
using environment variables and the current year.

### Example config

```yaml
token: "your_aoc_session_cookie_here"
year: 2024
```

Alternatively, you can set the `AOC_TOKEN` environment variable; the config
loader will use it when creating a new config.

## Features

- `aoc-client` (optional): enable remote input fetching and answer submission
  with the [`aoc-client`](https://crates.io/crates/aoc-client) crate.
- `test-helpers`: export a small testing API:
  - `aoc_star::test_helpers::CommandArgument`
  - `aoc_star::test_helpers::run_with_result`

These are useful for integration tests that want to bypass actual CLI parsing.

## Session cookie

To fetch puzzle inputs and submit answers, `aoc-star` needs your Advent of Code
session cookie.

You can find it by inspecting the cookies in your browser while logged in to
Advent of Code. Look for a cookie named `session`.
Set it in the config file as shown above, or set the `AOC_TOKEN` environment
variable before running your binary. You can see more details in the
[`aoc-cli` documentation](https://crates.io/crates/aoc-cli).

## License

This project is licensed under the MIT License and the Apache License (Version 2.0).
