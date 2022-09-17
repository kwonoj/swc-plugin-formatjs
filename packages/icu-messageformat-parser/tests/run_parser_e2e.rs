#![allow(non_snake_case)]
use icu_messageformat_parser::{AstElement, Error, Parser, ParserOptions};
use serde::{Serialize};
use std::{fs, path::PathBuf};
use testing::fixture;

#[derive(Debug)]
struct TestFixtureSections {
    message: String,
    snapshot_options: ParserOptions,
    expected: String,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct Snapshot<'a> {
    val: Vec<AstElement<'a>>,
    err: Option<Error>,
}

fn read_sections<'a>(file: PathBuf) -> TestFixtureSections {
    let input = fs::read_to_string(file).expect("Should able to read fixture");

    let input: Vec<&str> = input.split("\n---\n").collect();

    TestFixtureSections {
        message: input.get(0).expect("").to_string(),
        snapshot_options: serde_json::from_str(input.get(1).expect(""))
            .expect("Should able to deserialize options"),
        expected: input.get(2).expect("").to_string(),
    }
}

#[fixture("tests/fixtures/date_arg_skeleton_1")]
fn parser_tests(file: PathBuf) {
    let fixture_sections = read_sections(file);
    let mut parser = Parser::new(
        &fixture_sections.message,
        Some(&fixture_sections.snapshot_options),
    );

    let parsed_result = parser.parse();
    let parsed_result_snapshot = match parsed_result {
        Ok(parsed_result) => Snapshot {
            val: parsed_result,
            err: None,
        },
        Err(err) => Snapshot {
            val: vec![],
            err: Some(err),
        },
    };

    let parsed_result_str = serde_json::to_string_pretty(&parsed_result_snapshot).expect("Should able to serialize parsed result");
    similar_asserts::assert_eq!(parsed_result_str, fixture_sections.expected);
}
