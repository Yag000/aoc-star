mod cli;
mod config;
mod runner;

pub use aoc_star_derive::star;
use clap::Parser;
pub use inventory;

use crate::{
    cli::CommandArgument,
    runner::{config_year, run_day},
};

pub struct AocEntry {
    pub day: u32,
    pub part: u32,
    pub year: Option<i32>,
    pub func: fn(String) -> String,
}

inventory::collect!(AocEntry);

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let command_argument = CommandArgument::parse();

    let year = command_argument.year.unwrap_or(config_year());

    let day = command_argument.day;

    let part = command_argument.part.unwrap_or(1);

    let entry = inventory::iter::<AocEntry>()
        .find(|e| {
            e.day == day
                && e.part == part
                && (e.year == Some(year) || (e.year.is_none() && year == config_year()))
        })
        .ok_or_else(|| panic!("No solution found for day {day} part {part} of year {year}"))?;

    println!("Executing day {day} part {part} of year {year}");

    run_day(
        entry,
        command_argument.publish,
        &command_argument.input_file,
    )?;

    Ok(())
}
