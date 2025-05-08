use std::fs;
use std::io;
use std::path::PathBuf;

extern crate test_util_proc_macros;
pub use test_util_proc_macros::generate_tests;

pub struct TestFileContent {
    pub pine_content: String,
    pub tok_ref_content: Option<String>,
}

impl TestFileContent {
    fn new(pine_content: String, tok_ref_content: Option<String>) -> Self {
        Self {
            pine_content,
            tok_ref_content,
        }
    }
}

pub fn test_file_content(test_base_path: &mut PathBuf) -> io::Result<TestFileContent> {
    // read the test pine program
    test_base_path.set_extension("p");
    let pine_content = fs::read_to_string(test_base_path.as_path())?;
    
    // read the token ref if exists
    test_base_path.set_extension("tok");
    let token_ref_content = match fs::read_to_string(test_base_path.as_path()) {
        Ok(content) => Some(content),
        Err(_) => None,
    };

    Ok(TestFileContent::new(pine_content, token_ref_content))
}
