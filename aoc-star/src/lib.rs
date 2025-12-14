//! # AOC Star

mod cli;
mod config;
mod runner;

// Re-export the star macro
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
    pub use crate::cli::CommandArgument;
    pub use crate::runner::run_with_result;
}

pub struct AocEntry {
    pub day: u32,
    pub part: u32,
    pub year: Option<i32>,
    pub func: fn(String) -> String,
}

inventory::collect!(AocEntry);

/// Run the Advent of Code solution based on command line arguments
/// It should be used in the main function of the binary crate
/// ```no_run
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///    aoc_star::run()
///    }
/// ```
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    // We get the command line arguments
    let command_argument = cli::CommandArgument::parse();
    let result = run_with_result(command_argument)?;
    println!("{}", result);
    Ok(())
}
