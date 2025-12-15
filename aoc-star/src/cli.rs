//! Clap-based command line argument parser for the cli tool.
//! We force the user to provide the day and part to run.
//! If the part is not provided, it defaults to 1.

use clap::Parser;
#[derive(Parser, Clone, Debug)]
pub struct CommandArgument {
    #[clap(short, long, required_unless_present = "setup")]
    pub day: Option<u32>,

    #[clap(short, long, default_value_t = 1)]
    pub part: u32,

    #[clap(short, long)]
    pub year: Option<i32>,

    #[clap(long)]
    pub input_file: Option<String>,

    #[clap(long)]
    pub publish: bool,
    // Maybe in the future
    //#[clap(long)]
    //pub all: bool,
    #[clap(long)]
    pub setup: bool,
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use crate::cli::CommandArgument;

    #[test]
    fn parses_minimal_arguments() {
        // program name + required flags
        let args = ["aoc-star", "--day", "3"];
        let cmd = CommandArgument::parse_from(&args);

        assert_eq!(cmd.day, Some(3));
        assert_eq!(cmd.part, 1);
        assert_eq!(cmd.year, None);
        assert_eq!(cmd.input_file, None);
        assert!(!cmd.publish);
    }

    #[test]
    fn parses_all_arguments_long_flags() {
        let args = [
            "aoc-star",
            "--day",
            "5",
            "--part",
            "2",
            "--year",
            "2024",
            "--input-file",
            "input.txt",
            "--publish",
        ];
        let cmd = CommandArgument::parse_from(&args);

        assert_eq!(cmd.day, Some(5));
        assert_eq!(cmd.part, 2);
        assert_eq!(cmd.year, Some(2024));
        assert_eq!(cmd.input_file.as_deref(), Some("input.txt"));
        assert!(cmd.publish);
    }

    #[test]
    fn parses_short_flags() {
        let args = ["aoc-star", "-d", "10", "-p", "2", "-y", "2020"];
        let cmd = CommandArgument::parse_from(&args);

        assert_eq!(cmd.day, Some(10));
        assert_eq!(cmd.part, 2);
        assert_eq!(cmd.year, Some(2020));
    }

    #[test]
    fn fails_when_day_is_missing() {
        // clap will exit the process on error, but we can use try_parse_from
        let args = ["aoc-star", "--part", "2"];
        let result = CommandArgument::try_parse_from(&args);
        assert!(result.is_err());
    }
}
