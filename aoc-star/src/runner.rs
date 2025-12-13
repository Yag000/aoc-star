use aoc_client::AocClient;
use chrono::Datelike;

use crate::AocEntry;

fn get_year(entry: &AocEntry) -> i32 {
    entry.year.unwrap_or(chrono::Utc::now().year())
}

fn get_client(entry: &AocEntry) -> Result<AocClient, Box<dyn std::error::Error>> {
    let year = get_year(entry);
    let client = AocClient::builder()
        .session_cookie_from_default_locations()?
        .year(year)?
        .day(entry.day)?
        .build()?;
    Ok(client)
}

fn get_input(
    entry: &AocEntry,
    input_filename: &Option<String>,
) -> Result<String, Box<dyn std::error::Error>> {
    match input_filename {
        Some(filename) => {
            let input = std::fs::read_to_string(filename)?;
            Ok(input)
        }
        None => {
            let input = get_client(entry)?.get_input()?;
            Ok(input)
        }
    }
}

pub(crate) fn run_day(
    entry: &AocEntry,
    publish: bool,
    input_filename: &Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let result = (entry.func)(get_input(entry, input_filename)?);

    if publish {
        get_client(entry)?
            .submit_answer_and_show_outcome(&entry.part.to_string(), result)
            .unwrap();
    } else {
        println!("{}", result);
    }

    Ok(())
}
