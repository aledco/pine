extern crate proc_macro;
use proc_macro::TokenStream;
use std::fs;
use std::path::PathBuf;

#[proc_macro]
pub fn make_lex_single_file_tests(_input: TokenStream) -> TokenStream {
    create_output("lex").parse().unwrap()
}

#[proc_macro]
pub fn make_parse_single_file_tests(_input: TokenStream) -> TokenStream {
    create_output("parse").parse().unwrap()
}

fn create_output(name: &str) -> String {
    let n = single_file_tests_n();
    (1..=n)
        .into_iter()
        .map(|i| format!("#[test]\nfn test_{name}{i}() {{ test({i}); }}\n"))
        .collect::<String>()
}

fn single_file_tests_n() -> usize {
    let path = single_file_tests_path();
    fs::read_dir(path).unwrap().count()
}

fn single_file_tests_path() -> String {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../resources/tests/single_file/");
    path.to_str().unwrap().to_string()
}
