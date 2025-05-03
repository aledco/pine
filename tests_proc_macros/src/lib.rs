extern crate proc_macro;
use proc_macro::TokenStream;
use std::path::PathBuf;
use std::fs;

#[proc_macro]
pub fn make_lex_single_file_tests(_input: TokenStream) -> TokenStream {
    let n = single_file_tests_n();

    create_output(n, |i| format!("#[test]\nfn test_lex{i}() {{ test({i}); }}\n"))
        .parse()
        .unwrap()
}

fn create_output(n: usize, template: fn(usize) -> String) -> String {
    (1..=n)
        .into_iter()
        .map(|i| template(i))
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
