mod cli;
mod runner;

pub use aoc_star_derive::star;
use chrono::Datelike;
use clap::Parser;
pub use inventory;

use crate::{cli::CommandArgument, runner::run_day};

pub struct AocEntry {
    pub day: u32,
    pub part: u32,
    pub year: Option<i32>,
    pub func: fn(String) -> String,
}

inventory::collect!(AocEntry);

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let command_argument = CommandArgument::parse();

    let year = command_argument.year.unwrap_or(chrono::Utc::now().year());

    let day = command_argument.day;

    let part = command_argument.part.unwrap_or(1);

    let entry = inventory::iter::<AocEntry>()
        .find(|e| e.day == day && e.part == part && e.year == Some(year))
        .ok_or_else(|| panic!("No solution found for day {day} part {part}"))?;

    println!("Executing day {day} part {part} of year {year}");

    run_day(
        entry,
        command_argument.publish,
        &command_argument.input_filename,
    )?;

    Ok(())
}
