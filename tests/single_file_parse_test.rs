use ast::{lex::lex, parse::parse};

extern crate tests_proc_macros;
use tests_proc_macros::make_parse_single_file_tests;

mod common;

make_parse_single_file_tests!();

fn test(n: usize) {
    let input = read_test_files(n);
    let tokens = lex(input);
    let program = parse(tokens);
    //println!("{:#?}", program);
    assert!(program.functions.len() > 0);
}

fn read_test_files(i: usize) -> String {
    let mut path = common::test_file_base_path(i);
    common::read_pine_input(&mut path).expect("pine input is mising")
}
