extern crate proc_macro;
use proc_macro::TokenStream;
use std::fs;
use std::fs::DirEntry;
use std::path::PathBuf;

#[proc_macro]
pub fn make_lex_single_file_tests(_input: TokenStream) -> TokenStream {
    create_output("lex").parse().unwrap()
}

#[proc_macro]
pub fn make_parse_single_file_tests(_input: TokenStream) -> TokenStream {
    create_output("parse").parse().unwrap()
}

#[proc_macro]
pub fn make_traverse_single_file_tests(_input: TokenStream) -> TokenStream {
    create_output("traverse").parse().unwrap()
}


fn create_output(name: &str) -> String {
    let path = single_file_tests_path();
    fs::read_dir(path)
        .unwrap()
        .map(|d| d.unwrap())
        .map(|d| test_types(d))
        .map(|(tt, ds)| create_tests(name, &tt, ds))
        .flatten()
        .collect::<String>()
}

fn test_types(d: DirEntry) -> (String, Vec<DirEntry>) {
    let base_path = d.path();
    let test_type = base_path
        .iter()
        .last()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let test_dirs = base_path.read_dir().unwrap().map(|d| d.unwrap()).collect();
    (test_type, test_dirs)
}

fn create_tests(name: &str, test_type: &str, test_dirs: Vec<DirEntry>) -> Vec<String> {
    test_dirs
        .into_iter()
        .map(|d| {
            let base_path = d.path();
            let test_name = base_path.iter().last().unwrap().to_str().unwrap();
            let test_path = base_path.to_str().unwrap();
            format!(
                "#[test]\nfn {}_{}_{}() {{ test(\"{}\"); }}\n",
                name, test_type, test_name, test_path
            )
        })
        .collect()
}

fn single_file_tests_path() -> String {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../resources/tests/single_file");
    path.to_str().unwrap().to_string()
}
