use chrono::Datelike;

#[allow(unused_imports)]
use crate::{AocEntry, config::get_config, config::get_config_token};

#[cfg(feature = "aoc-client")]
use aoc_client::AocClient;

pub(crate) fn config_year() -> i32 {
    get_config()
        .year
        .unwrap_or_else(|| chrono::Utc::now().year())
}

#[allow(dead_code)]
fn resolve_year(entry: &AocEntry) -> i32 {
    entry.year.unwrap_or_else(config_year)
}

#[cfg(feature = "aoc-client")]
fn get_cookie() -> Result<String, Box<dyn std::error::Error>> {
    let config_cookie = get_config_token();
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
fn build_aoc_client(entry: &AocEntry) -> Result<AocClient, Box<dyn std::error::Error>> {
    let cookie = get_cookie()?;

    Ok(AocClient::builder()
        .session_cookie(cookie)?
        .year(resolve_year(entry))?
        .day(entry.day)?
        .build()?)
}

fn get_input(
    entry: &AocEntry,
    input_file: &Option<String>,
) -> Result<String, Box<dyn std::error::Error>> {
    match input_file {
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
