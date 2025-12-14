use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitInt, Result, Token};

/// Struct representing the parsed arguments for the `star` macro.
#[derive(Debug)]
pub struct AocArgs {
    pub day: u32,
    pub part: u32,
    pub year: Option<i32>,
}

impl Parse for AocArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut day = None;
        let mut part = None;
        let mut year = None;

        while !input.is_empty() {
            let ident: Ident = input.parse()?;

            input.parse::<Token![=]>()?;

            if ident == "day" {
                let value: LitInt = input.parse()?;
                day = Some(value.base10_parse()?);
            } else if ident == "part" {
                let value: LitInt = input.parse()?;
                part = Some(value.base10_parse()?);
            } else if ident == "year" {
                let value: LitInt = input.parse()?;
                year = Some(value.base10_parse()?);
            } else {
                return Err(syn::Error::new_spanned(
                    ident,
                    "unknown attribute key (expected day, part, or year)",
                ));
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(AocArgs {
            day: day.ok_or_else(|| input.error("missing `day`"))?,
            part: part.ok_or_else(|| input.error("missing `part`"))?,
            year,
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_str;

    fn parse(input: &str) -> AocArgs {
        parse_str::<AocArgs>(input).unwrap_or_else(|e| panic!("expected OK, got error: {e}"))
    }

    fn assert_parse_ok(input: &str, expected: &AocArgs) {
        let parsed = parse(input);

        assert_eq!(parsed.day, expected.day);
        assert_eq!(parsed.part, expected.part);
        assert_eq!(parsed.year, expected.year);
    }

    fn assert_tests(test: &[(&str, AocArgs)]) {
        test.iter()
            .for_each(|(input, expected)| assert_parse_ok(input, expected));
    }

    fn assert_parse_fails(input: &str) {
        if parse_str::<AocArgs>(input).is_ok() {
            panic!(
                "expected parsing to fail, but it succeeded: `{input}` {:?}",
                parse_str::<AocArgs>(input)
            );
        }
    }

    fn assert_fail_tests(test: &[&str]) {
        test.iter().for_each(|input| assert_parse_fails(input));
    }

    #[test]
    fn parses_day_and_part_only() {
        let tests = [
            (
                "day = 1, part = 1",
                AocArgs {
                    day: 1,
                    part: 1,
                    year: None,
                },
            ),
            (
                "day = 25, part = 2",
                AocArgs {
                    day: 25,
                    part: 2,
                    year: None,
                },
            ),
        ];

        assert_tests(&tests);
    }

    #[test]
    fn parses_with_year() {
        let tests = [
            (
                "day = 1, part = 1, year = 2023",
                AocArgs {
                    day: 1,
                    part: 1,
                    year: Some(2023),
                },
            ),
            (
                "day = 10, part = 2, year = 2022",
                AocArgs {
                    day: 10,
                    part: 2,
                    year: Some(2022),
                },
            ),
        ];

        assert_tests(&tests);
    }

    #[test]
    fn parses_arguments_in_any_order() {
        let tests = [
            (
                "part = 2, day = 3",
                AocArgs {
                    day: 3,
                    part: 2,
                    year: None,
                },
            ),
            (
                "year = 2021, part = 1, day = 7",
                AocArgs {
                    day: 7,
                    part: 1,
                    year: Some(2021),
                },
            ),
        ];

        assert_tests(&tests);
    }

    #[test]
    fn allows_trailing_comma() {
        let tests = [
            (
                "day = 1, part = 1,",
                AocArgs {
                    day: 1,
                    part: 1,
                    year: None,
                },
            ),
            (
                "day = 5, part = 2, year = 2023,",
                AocArgs {
                    day: 5,
                    part: 2,
                    year: Some(2023),
                },
            ),
        ];

        assert_tests(&tests);
    }

    #[test]
    fn parses_equivalent_numeric_literals() {
        let tests = [
            (
                "day = 01, part = 01",
                AocArgs {
                    day: 1,
                    part: 1,
                    year: None,
                },
            ),
            (
                "day = 0xA, part = 0b10",
                AocArgs {
                    day: 10,
                    part: 2,
                    year: None,
                },
            ),
            (
                "day = 1_0, part = 2_0",
                AocArgs {
                    day: 10,
                    part: 20,
                    year: None,
                },
            ),
            (
                "day = 0o12, part = 2",
                AocArgs {
                    day: 10,
                    part: 2,
                    year: None,
                },
            ),
        ];

        assert_tests(&tests);
    }

    #[test]
    fn parses_non_comma_lists() {
        let tests = [
            (
                "day = 1 part = 2",
                AocArgs {
                    day: 1,
                    part: 2,
                    year: None,
                },
            ),
            (
                "part = 2, day = 1",
                AocArgs {
                    day: 1,
                    part: 2,
                    year: None,
                },
            ),
            (
                "day = 1 part = 2 year = 2020",
                AocArgs {
                    day: 1,
                    part: 2,
                    year: Some(2020),
                },
            ),
            (
                "day = 1 year = 2020 part = 2",
                AocArgs {
                    day: 1,
                    part: 2,
                    year: Some(2020),
                },
            ),
            (
                "year = 2020 day = 1 part = 2",
                AocArgs {
                    day: 1,
                    part: 2,
                    year: Some(2020),
                },
            ),
        ];

        assert_tests(&tests);
    }

    #[test]
    fn parses_repeated() {
        let tests = [
            (
                "day = 1, day =5, part = 2",
                AocArgs {
                    day: 5,
                    part: 2,
                    year: None,
                },
            ),
            (
                "day = 1, day =5, day = 7, part = 2",
                AocArgs {
                    day: 7,
                    part: 2,
                    year: None,
                },
            ),
            (
                "day = 1, day =5, part = 2, part = 7",
                AocArgs {
                    day: 5,
                    part: 7,
                    year: None,
                },
            ),
            (
                "day = 1, day =5, part = 2, part = 7, year = 2021",
                AocArgs {
                    day: 5,
                    part: 7,
                    year: Some(2021),
                },
            ),
            (
                "year = 2003, day = 1, day =5, part = 2, part = 7, year = 2021",
                AocArgs {
                    day: 5,
                    part: 7,
                    year: Some(2021),
                },
            ),
            (
                "year = 2003, day = 1, day =5, part = 2, part = 7, year = 2021, day = 8",
                AocArgs {
                    day: 8,
                    part: 7,
                    year: Some(2021),
                },
            ),
        ];

        assert_tests(&tests);
    }

    #[test]
    fn rejects_ill_formed_inputs() {
        let tests = [
            "day 1, part 1",
            "day = 1, day = 1",
            "day = a, part = 1",
            "day = 1, part = b",
            "day = 1, part = 1, year = b",
            "day = , part = 1",
            "day = 1",
            "part = 1",
            "day == 1, part = 2",
            "day = 1, part =",
            "day = one, part = 2",
            "day = 1, part = 2,,",
            "day = 1, part = 2, year =",
            "day = 1, part = 2, year = \"2023\"",
        ];

        assert_fail_tests(&tests);
    }
}
