use std::fs;
use std::path::PathBuf;

extern crate test_util_proc_macros;
pub use test_util_proc_macros::{generate_pvm_tests, generate_single_file_tests};

pub struct TestFileContent {
    pub pine_content: Option<String>,
    pub tok_ref_content: Option<String>,
    pub pvm_content: Option<String>,
    pub pvm_ref_content: Option<String>,
    pub test_input_file: Option<fs::File>,
}

impl TestFileContent {
    fn new(
        pine_content: Option<String>,
        tok_ref_content: Option<String>,
        pvm_content: Option<String>,
        pvm_ref_content: Option<String>,
        test_input_file: Option<fs::File>,
    ) -> Self {
        Self {
            pine_content,
            tok_ref_content,
            pvm_content,
            pvm_ref_content,
            test_input_file
        }
    }
}

pub fn test_file_content(test_base_path: &mut PathBuf) -> TestFileContent {
    // read the test pine program
    test_base_path.set_extension("p");
    let pine_content = match fs::read_to_string(test_base_path.as_path()) {
        Ok(content) => Some(content),
        Err(_) => None,
    };

    // read the test pvm program
    test_base_path.set_extension("pvm");
    let pvm_content = match fs::read_to_string(test_base_path.as_path()) {
        Ok(content) => Some(content),
        Err(_) => None,
    };

    // read the token ref if exists
    test_base_path.set_extension("tok");
    let token_ref_content = match fs::read_to_string(test_base_path.as_path()) {
        Ok(content) => Some(content),
        Err(_) => None,
    };

    // read the test input if exists
    test_base_path.set_extension("in");
    let test_input_file = match fs::File::open(test_base_path.as_path()) {
        Ok(file) => Some(file),
        Err(_) => None,
    };
    
    // read the pvm ref if exists
    test_base_path.set_extension("pvm.ref");
    let pvm_ref_content = match fs::read_to_string(test_base_path.as_path()) {
        Ok(content) => Some(content),
        Err(_) => None,
    };

    TestFileContent::new(
        pine_content,
        token_ref_content,
        pvm_content,
        pvm_ref_content,
        test_input_file
    )
}
