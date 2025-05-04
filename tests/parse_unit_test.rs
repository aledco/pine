use std::path::PathBuf;
use std::str::FromStr;
use ast::{lex::lex, parse::parse};

extern crate tests_proc_macros;
use tests_proc_macros::make_parse_single_file_tests;

mod common;

make_parse_single_file_tests!();

fn test(test_base_path: &str) {
    let input = read_test_files(PathBuf::from_str(test_base_path).unwrap());
    let tokens = lex(input);
    let program = parse(tokens);
    println!("{:#?}", program);
    assert!(program.functions.len() > 0);
}

fn read_test_files(mut base_path: PathBuf) -> String {
    base_path.push("test");
    common::read_pine_input(&mut base_path).expect("pine input is mising")
}
