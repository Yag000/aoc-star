use chrono::Datelike;

use crate::cli::CommandArgument;

use crate::{AocEntry, config::get_config};

#[cfg(feature = "aoc-client")]
use crate::config::get_config_token;
#[cfg(feature = "aoc-client")]
use aoc_client::AocClient;

pub fn run_with_result(
    command_argument: CommandArgument,
) -> Result<String, Box<dyn std::error::Error>> {
    let year = command_argument
        .year
        .unwrap_or_else(|| config_year().unwrap());

    let day = command_argument.day.expect("Day is required");

    let part = command_argument.part;

    // We look for the corresponding entry in the inventory
    // If no specific year match is found, we fall back to year-agnostic solutions
    // If no solution is found, we panic
    let mut entry: Option<&AocEntry> = None;
    for e in crate::inventory::iter::<AocEntry> {
        if e.day == day && e.part == part {
            if let Some(e_year) = e.year {
                if e_year == year {
                    entry = Some(e);
                    break;
                }
            } else if entry.is_none() {
                entry = Some(e);
            }
        }
    }

    let entry = entry
        .unwrap_or_else(|| panic!("No solution found for Day {day} Part {part} of Year {year}"));

    println!("Executing Day {day} Part {part} of Year {year}");

    // We run the day with the provided arguments
    run_day(
        entry,
        command_argument.publish,
        &command_argument.input_file,
    )
}
/// Retrieves the configured year or defaults to the current year.
pub(crate) fn config_year() -> Result<i32, Box<dyn std::error::Error>> {
    Ok(get_config()?
        .year
        .unwrap_or_else(|| chrono::Utc::now().year()))
}

/// Executes the given AocEntry with the provided options.
/// If `publish` is true, the result will be submitted to Advent of Code,
/// but only if the `aoc-client` feature is enabled.
pub(crate) fn run_day(
    entry: &AocEntry,
    publish: bool,
    input_file: &Option<String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let result = (entry.func)(get_input(entry, input_file)?);

    if publish {
        publish_result(entry, &result)?;
    }

    Ok(result)
}
/// Retrieves the input for the given AocEntry.
/// If `input_file` is provided, reads the input from the file.
/// Otherwise, fetches the input remotely using the aoc-client crate
fn get_input(
    entry: &AocEntry,
    input_file: &Option<String>,
) -> Result<String, Box<dyn std::error::Error>> {
    match input_file {
        Some(filename) => Ok(std::fs::read_to_string(filename)?),
        None => get_remote_input(entry),
    }
}
#[cfg(not(feature = "aoc-client"))]
fn get_remote_input(_: &AocEntry) -> Result<String, Box<dyn std::error::Error>> {
    panic!(
        "Remote input is disabled. \
         Enable the `aoc-client` feature or provide an input file."
    );
}

#[cfg(feature = "aoc-client")]
fn get_remote_input(entry: &AocEntry) -> Result<String, Box<dyn std::error::Error>> {
    // TODO: write it up somewhere and check every time if it's there (maybe
    // add some check to test someones hasnt changed it etc, a flag or something like that)
    // before downloading ect. Probably use the input/ dir (create it if needed)

    use std::path::PathBuf;

    let mut path = PathBuf::from("input");
    std::fs::create_dir_all(&path)?;
    path.push(format!("{}{}.txt", entry.day, resolve_year(entry)?));

    if path.exists() {
        let input = std::fs::read_to_string(&path)?;

        return Ok(input);
    }

    let input = build_aoc_client(entry)?.get_input()?;
    std::fs::write(path, &input)?;

    Ok(input)
}

#[cfg(feature = "aoc-client")]
/// Builds an AocClient for the given AocEntry.
fn build_aoc_client(entry: &AocEntry) -> Result<AocClient, Box<dyn std::error::Error>> {
    let cookie = get_cookie()?;

    Ok(AocClient::builder()
        .session_cookie(cookie)?
        .year(resolve_year(entry)?)?
        .day(entry.day)?
        .build()?)
}
#[cfg(feature = "aoc-client")]
/// Retrieves the session cookie from the config file or environment variable.
fn get_cookie() -> Result<String, Box<dyn std::error::Error>> {
    let config_cookie = get_config_token()?;
    if !config_cookie.is_empty() {
        Ok(config_cookie)
    } else if let Ok(env_cookie) = std::env::var("AOC_TOKEN").as_ref() {
        Ok(env_cookie.clone())
    } else {
        Err(Box::new(std::io::Error::other(
            "AOC session cookie is missing. Please set it in the config file or AOC_TOKEN environment variable.",
        )))
    }
}
#[cfg(feature = "aoc-client")]
/// Resolves the year for the given AocEntry.
fn resolve_year(entry: &AocEntry) -> Result<i32, Box<dyn std::error::Error>> {
    match entry.year {
        Some(year) => Ok(year),
        None => config_year(),
    }
}

#[cfg(not(feature = "aoc-client"))]
/// Stub function for publishing results when the aoc-client feature is disabled.
fn publish_result(_: &AocEntry, _: &str) -> Result<(), Box<dyn std::error::Error>> {
    panic!("Publishing answers requires the `aoc-client` feature to be enabled.");
}

#[cfg(feature = "aoc-client")]
/// Publishes the result to Advent of Code using the aoc-client crate.
fn publish_result(entry: &AocEntry, result: &str) -> Result<(), Box<dyn std::error::Error>> {
    build_aoc_client(entry)?.submit_answer_and_show_outcome(&entry.part.to_string(), result)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use crate::AocEntry;
    use tempfile::NamedTempFile;

    #[test]
    fn get_input_reads_from_file_when_provided() {
        let mut tmp = NamedTempFile::new().unwrap();
        tmp.write_fmt(format_args!("hello world")).unwrap();
        let path = tmp.path().to_str().unwrap().to_string();

        let entry = AocEntry {
            day: 1,
            part: 1,
            year: None,
            func: |s| s,
        };

        let input = super::get_input(&entry, &Some(path)).unwrap();
        assert!(input.contains("hello world"));
    }

    #[test]
    #[should_panic(expected = "Remote input is disabled")]
    #[cfg(not(feature = "aoc-client"))]
    fn get_input_panics_without_aoc_client_and_no_file() {
        use crate::runner::get_input;

        let entry = AocEntry {
            day: 1,
            part: 1,
            year: None,
            func: |s| s,
        };
        // This should call get_remote_input and panic
        let _ = get_input(&entry, &None);
    }

    #[test]
    #[should_panic(
        expected = "Publishing answers requires the `aoc-client` feature to be enabled."
    )]
    #[cfg(not(feature = "aoc-client"))]
    fn publish_result_panics_without_aoc_client() {
        use crate::runner::publish_result;

        let entry = AocEntry {
            day: 1,
            part: 1,
            year: None,
            func: |s| s,
        };
        let _ = publish_result(&entry, "42");
    }
}
