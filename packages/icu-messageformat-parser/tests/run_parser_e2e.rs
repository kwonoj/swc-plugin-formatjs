#![allow(non_snake_case)]
use icu_messageformat_parser::{Parser, ParserOptions};
use std::{fs, path::PathBuf};
use testing::fixture;

#[derive(Debug)]
struct TestFixtureSections {
    message: String,
    snapshot_options: ParserOptions,
    expected: String,
}

fn read_sections(file: PathBuf) -> TestFixtureSections {
    let input = fs::read_to_string(file).expect("Should able to read fixture");

    let input: Vec<&str> = input.split("\n---\n").collect();

    TestFixtureSections {
        message: input.get(0).expect("").to_string(),
        snapshot_options: serde_json::from_str(input.get(1).expect("")).expect("Should able to deserialize options"),
        expected: input.get(2).expect("").to_string(),
    }
}

#[fixture("tests/fixtures/date_arg_skeleton_1")]
fn parser_tests(file: PathBuf) {
    let fixture_sections = read_sections(file);
    let parser = Parser::new(&fixture_sections.message, None);

    println!("{:#?}", fixture_sections);
}
