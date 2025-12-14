use chrono::Datelike;

use crate::AocEntry;

#[cfg(feature = "aoc-client")]
use aoc_client::AocClient;

#[allow(dead_code)]
fn resolve_year(entry: &AocEntry) -> i32 {
    entry.year.unwrap_or_else(|| chrono::Utc::now().year())
}

#[cfg(feature = "aoc-client")]
fn build_aoc_client(entry: &AocEntry) -> Result<AocClient, Box<dyn std::error::Error>> {
    Ok(AocClient::builder()
        .session_cookie_from_default_locations()?
        .year(resolve_year(entry))?
        .day(entry.day)?
        .build()?)
}

fn get_input(
    entry: &AocEntry,
    input_file: &Option<String>,
) -> Result<String, Box<dyn std::error::Error>> {
    match input_file{
        Some(filename) => Ok(std::fs::read_to_string(filename)?),
        None => get_remote_input(entry),
    }
}

#[cfg(feature = "aoc-client")]
fn get_remote_input(entry: &AocEntry) -> Result<String, Box<dyn std::error::Error>> {
    Ok(build_aoc_client(entry)?.get_input()?)
}

#[cfg(not(feature = "aoc-client"))]
fn get_remote_input(_: &AocEntry) -> Result<String, Box<dyn std::error::Error>> {
    panic!(
        "Remote input is disabled. \
         Enable the `aoc-client` feature or provide an input file."
    );
}

pub(crate) fn run_day(
    entry: &AocEntry,
    publish: bool,
    input_file: &Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let result = (entry.func)(get_input(entry, input_file)?);

    if publish {
        publish_result(entry, &result)?;
    } else {
        println!("{}", result);
    }

    Ok(())
}

#[cfg(feature = "aoc-client")]
fn publish_result(entry: &AocEntry, result: &str) -> Result<(), Box<dyn std::error::Error>> {
    build_aoc_client(entry)?.submit_answer_and_show_outcome(&entry.part.to_string(), result)?;
    Ok(())
}

#[cfg(not(feature = "aoc-client"))]
fn publish_result(_: &AocEntry, _: &str) -> Result<(), Box<dyn std::error::Error>> {
    panic!("Publishing answers requires the `aoc-client` feature to be enabled.");
}
