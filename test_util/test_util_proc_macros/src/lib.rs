extern crate proc_macro;
use proc_macro::TokenStream;
use std::fs;
use std::fs::DirEntry;
use std::path::PathBuf;
use quote::{quote, format_ident};

#[proc_macro]
pub fn generate_tests(input: TokenStream) -> TokenStream {
    let name = input.to_string();
    create_output(&name)
}


fn create_output(name: &str) -> TokenStream {
    let path = single_file_tests_path();
    let test_fns: Vec<TokenStream> = fs::read_dir(path)
        .unwrap()
        .map(|d| d.unwrap())
        .map(|d| test_types(d))
        .map(|(tt, ds)| create_tests(name, &tt, ds))
        .flatten()
        .collect();

    let mut tokens = TokenStream::new();
    for test_fn in test_fns {
        tokens.extend(test_fn);
    }
    tokens
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

fn create_tests(name: &str, test_type: &str, test_dirs: Vec<DirEntry>) -> Vec<TokenStream> {
    test_dirs
        .into_iter()
        .map(|d| {
            let base_path = d.path();
            let test_name = base_path.iter().last().unwrap().to_str().unwrap();
            let test_base_path = base_path.to_str().unwrap().to_owned() + "/test";
            let test_fn_name = format_ident!("{}_{}_{}", name, test_type, test_name);
            quote! {
                #[test]
                fn #test_fn_name() {
                    let test_base_path = std::path::PathBuf::from(#test_base_path);
                    test(test_base_path)
                }
            }.into()
        })
        .collect()
}

fn single_file_tests_path() -> String {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../../resources/tests/single_file");
    path.to_str().unwrap().to_string()
}
