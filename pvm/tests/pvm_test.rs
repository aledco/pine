mod common;

use std::cell::RefCell;
use pvm::{parse, ExecuteConfig, execute_with_config};
use std::path::PathBuf;
use std::rc::Rc;
use test_util::{generate_pvm_tests, test_file_content};

generate_pvm_tests!(pvm);

const MEMORY: usize = 1024 * 1024;
fn test(mut test_base_path: PathBuf) {
    let test_content = test_file_content(&mut test_base_path);
    let pvm_content = test_content.pvm_content.expect("pvm test input is required");
    let instructions = parse(&pvm_content).unwrap();

    let buffer: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));
    let config = ExecuteConfig::new(MEMORY, buffer.clone());
    execute_with_config(instructions, config).unwrap();
    
    if let Some(pvm_ref_content) = test_content.pvm_ref_content {
        let actual: String = buffer
            .borrow()
            .iter()
            .map(|b| *b as char)
            .collect();
        equal(pvm_ref_content, actual);    
    } else {
        assert!(true);
    }
}

fn equal(expected: String, actual: String) {
    let expected = expected.lines().collect::<Vec<_>>();
    let actual = actual.lines().collect::<Vec<_>>();
    assert_eq!(expected, actual);
}
