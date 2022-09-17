#![allow(non_snake_case)]
use std::{fs, path::PathBuf};
use icu_messageformat_parser::Parser;
use testing::fixture;

#[derive(Debug)]
struct TestFixtureSections {
    message: String,
    snapshot_options: String,
    expected: String,
}

fn read_sections(file: PathBuf) -> TestFixtureSections {
    let input = fs::read_to_string(file).expect("Should able to read fixture");

    let input: Vec<&str> = input.split("\n---\n").collect();

    TestFixtureSections {
        message: input.get(0).expect("").to_string(),
        snapshot_options: input.get(1).expect("").to_string(),
        expected: input.get(2).expect("").to_string(),
    }
}


#[fixture("tests/fixtures/date_arg_skeleton_with_jjj")]
fn parser_tests(file: PathBuf) {
    let fixture_sections = read_sections(file);
    let parser = Parser::new(&fixture_sections.message, None);
}
