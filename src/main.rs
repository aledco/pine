use std::fs;
use std::fs::{DirEntry, ReadDir};
use std::path::PathBuf;

fn main() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("resources/tests/single_file");

    let name = "lex";
    let tests = fs::read_dir(path)
        .unwrap()
        .map(|d| d.unwrap())
        .map(|d| test_types(d))
        .map(|(tt, ds)| create_tests(name, &tt, ds))
        .flatten()
        .collect::<String>();

    println!("{}", tests);
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
            let dir_name = base_path.iter().last().unwrap().to_str().unwrap();
            format!(
                "#[test]\nfn test_{}_{}_{}() {{ test(\"{}\"); }}\n",
                name,
                test_type,
                dir_name,
                base_path.to_str().unwrap()
            )
        })
        .collect()
}
