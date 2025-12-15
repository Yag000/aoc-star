//! # AOC Star
//!
//! `aoc-star` is a small library and CLI helper to organize and run your
//! [Advent of Code](https://adventofcode.com/) solutions.
//!
//! It provides:
//!
//! - An attribute macro `#[aoc_star::star(...)]` to register solution functions
//!   for a given `day`, `part`, and optional `year`.
//! - A tiny “runner” that looks up the appropriate solution based on CLI
//!   arguments and executes it, wiring up input loading and (optionally)
//!   answer submission via `aoc-client`.
//!
//! ## Quick example
//!
//! Add `aoc-star` to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! aoc-star = "0.1"
//! ```
//!
//! Then in your binary crate (for example `src/bin/aoc.rs`):
//!
//! ```no_run
//! use aoc_star::star;
//!
//! #[star(day = 1, part = 1, year = 2024)]
//! fn day1_part1(input: String) -> String {
//!     // Solve the puzzle here using the contents of `input`
//!     input.lines().count().to_string()
//! }
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     aoc_star::run()
//! }
//! ```
//!
//! Running this binary like
//!
//! ```sh
//! cargo run -- --day 1 --part 1 --year 2024
//! ```
//!
//! will look up the registered solution for day 1, part 1, year 2024, fetch
//! the input (or read it from a file if you pass `--input-file`), and print
//! the resulting answer.
//!
//! ## Features
//!
//! - `aoc-client` (optional): enable remote input fetching and answer
//!   submission using the [`aoc-client`](https://crates.io/crates/aoc-client)
//!   crate. When disabled you must always provide an `--input-file`.
//! - `test-helpers`: re-exports some internals (`CommandArgument` and
//!   `run_with_result`) to make integration testing easier.
//!
//!! ### `star` macro
//!
//! The `#[star(day = X, part = Y, year = Z)]` attribute macro registers
//! the annotated function as the solution for the specified day, part, and
//! optional year. The function must have the signature `fn(String) -> String`,
//! where the input `String` contains the puzzle input, and the returned
//! `String` is the answer.
//!
//! If the `year` parameter is omitted, the solution is considered
//! year-agnostic and will be used for any year that does not halve a more specific solution.
//!
//! ### CLI arguments
//!
//! The `run` function parses the following command line arguments:
//!
//! - `--day <DAY>`: The Advent of Code day (1–25).
//! - `--part <PART>`: The puzzle part (usually 1 or 2).
//! - `--year <YEAR>`: The Advent of Code year (e.g., 2024). Defaults to
//!   the current year if not provided.
//! - `--input-file <FILE>`: Path to a file containing the puzzle input.
//!   This argument is required if the `aoc-client` feature is not enabled.
//! - `--publish`: If provided and the `aoc-client` feature is enabled,
//!   the computed answer will be submitted to Advent of Code.
//!
//! The default year is either the one on the config file or the current year.
//! The config contains the session cookie needed to fetch inputs and publish answers and
//! the default year. If not present, the config file can be created by running
//! this command with the `--setup` flag. It will be located at `$XDG_CONFIG_HOME/aoc-star/config.toml`
//! which in linux systems usually resolves to `~/.config/aoc-star/config.toml`.
//!
//! ## License

mod cli;
mod config;
mod runner;

// Re-export the star macro so users can just `use aoc_star::star;`.
pub use aoc_star_derive::star;

use clap::Parser;
// This re-export is unfortunately necessary because
// the macro expansions of the `aoc_star_derive::star` macro
// need to access the `inventory` crate from the same namespace
// as this crate. There may be a better way to handle this in the future.
pub use inventory;

use crate::runner::run_with_result;

#[cfg(any(test, feature = "test-helpers"))]
pub mod test_helpers {
    //! Helpers intended for testing `aoc-star` or crates that use it.
    //!
    //! This module is only available when the `test-helpers` feature is
    //! enabled
    //!
    //! It re-exports:
    //! - [`CommandArgument`](crate::cli::CommandArgument): the parsed CLI
    //!   arguments structure.
    //! - [`run_with_result`](crate::runner::run_with_result): the
    //!   programmatic entry point used by [`run`].
    pub use crate::cli::CommandArgument;
    pub use crate::runner::run_with_result;
}

/// A registered Advent of Code solution.
///
/// Instances of this type are created by the `#[star]` attribute macro in the
/// `aoc-star-derive` crate and collected via the [`inventory`] crate. You
/// usually do not construct this type directly.
///
/// - `day`: The Advent of Code day (1–25).
/// - `part`: The part of the puzzle (1 or 2).
/// - `year`: The Advent of Code year; if `None`, the solution is considered
///   year-agnostic and will be used for any year that does not have a
///   more specific solution.
/// - `func`: The solution function, which must take the puzzle input as a
///   `String` and return the answer as a `String`.
pub struct AocEntry {
    /// Advent of Code day number (1–25).
    pub day: u32,
    /// Puzzle part (usually `1` or `2`).
    pub part: u32,
    /// Advent of Code year, or `None` for year-agnostic solutions.
    pub year: Option<i32>,
    /// The solution function that processes the puzzle input and returns the answer.
    pub func: fn(String) -> String,
}

inventory::collect!(AocEntry);

/// Run the Advent of Code solution based on command line arguments.
///
/// This should be used in the `main` function of the binary crate that wires
/// together your solutions. It:
///
/// 1. Parses the command line using [`clap`].
/// 2. Locates the registered solution for the requested day/part/year.
/// 3. Loads the input (either from `--input-file` or, if the `aoc-client`
///    feature is enabled, remotely from Advent of Code).
/// 4. Optionally publishes the answer when `--publish` is used and
///    the `aoc-client` feature is enabled.
/// 5. Prints the resulting answer to stdout.
///
/// # Errors
///
/// Returns an error if input reading or (when enabled) communication with
/// Advent of Code fails. If no matching solution is found, this function
/// will panic.
///
/// # Examples
///
/// ```no_run
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     aoc_star::run()
/// }
/// ```
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    // We get the command line arguments
    let command_argument = cli::CommandArgument::parse();
    if command_argument.setup {
        config::setup_config_prompt()?;
        println!("Configuration file created successfully.");
        return Ok(());
    }
    let result = run_with_result(command_argument)?;
    println!("{result}");
    Ok(())
}
