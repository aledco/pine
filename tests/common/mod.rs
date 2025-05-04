use std::fs;
use std::io;
use std::path::PathBuf;

pub fn single_file_test_base_path(i: usize) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(format!("resources/tests/single_file/test{}/test", i));
    path
}

pub fn read_pine_input(base_path: &mut PathBuf) -> io::Result<String> {
    base_path.set_extension("p");
    fs::read_to_string(base_path.as_path())
}

pub fn read_token_ref(base_path: &mut PathBuf) -> io::Result<String> {
    base_path.set_extension("tok");
    fs::read_to_string(base_path.as_path())
}
