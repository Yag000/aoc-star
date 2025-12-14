use aoc_star::star;
use aoc_star::test_helpers::CommandArgument;
use aoc_star::test_helpers::run_with_result;
use tempfile::NamedTempFile;

#[star(day = 3, part = 1, year = 2024)]
fn example_day3_part1(_input: String) -> String {
    "answer-3-1-2024".to_string()
}

#[star(day = 3, part = 2, year = 2024)]
fn example_day3_part2_2024(_input: String) -> String {
    "answer-3-2-2024".to_string()
}

#[star(day = 1, part = 1)]
fn example_day3_part1_none(_input: String) -> String {
    "answer-3-2-none".to_string()
}

fn generate_dummy_file(str: &str) -> NamedTempFile {
    let tmp = NamedTempFile::new().unwrap();
    let path = tmp.path().to_path_buf();
    std::fs::write(&path, str).unwrap();
    tmp
}

#[test]
fn test_run_day3_part1_2024() {
    let tmp = generate_dummy_file("dummy input");

    let args = CommandArgument {
        day: 3,
        part: 1,
        year: Some(2024),
        input_file: Some(tmp.path().to_str().unwrap().to_string()),
        publish: false,
    };

    let result = run_with_result(args).unwrap();

    assert_eq!(result.trim(), "answer-3-1-2024");
}

#[test]
fn test_run_day3_part2_2024() {
    let tmp = generate_dummy_file("dummy input");

    let args = CommandArgument {
        day: 3,
        part: 2,
        year: Some(2024),
        input_file: Some(tmp.path().to_str().unwrap().to_string()),
        publish: false,
    };

    let result = run_with_result(args).unwrap();

    assert_eq!(result.trim(), "answer-3-2-2024");
}

#[test]
fn test_run_day1_part1_none() {
    let tmp = generate_dummy_file("dummy input");

    let args = CommandArgument {
        day: 1,
        part: 1,
        year: Some(2025),
        input_file: Some(tmp.path().to_str().unwrap().to_string()),
        publish: false,
    };

    let result = run_with_result(args).unwrap();

    assert_eq!(result.trim(), "answer-3-2-none");
}

#[test]
#[should_panic(expected = "No solution found for Day 2 Part 1 of Year 2018")]
fn test_run_non_existent_solution() {
    let args = CommandArgument {
        day: 2,
        part: 1,
        year: Some(2018),
        input_file: None,
        publish: false,
    };
    let _ = run_with_result(args).unwrap();
}
