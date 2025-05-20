mod common;

use std::path::PathBuf;
use test_util::{generate_single_file_tests, test_file_content};

generate_single_file_tests!(ast);

fn test(mut test_base_path: PathBuf) {
    let test_content = test_file_content(&mut test_base_path);
    let pine_content = test_content.pine_content.expect("pine test input is required");
    if let Some(token_ref) = test_content.tok_ref_content {
        // test the lexer
        let actual = ast::lex::lex(pine_content.clone()).unwrap();
        let expected = common::parse_token_ref(token_ref);
        assert_eq!(expected, actual);
    }

    let program = ast::parse(pine_content).unwrap();
    assert!(program.functions.len() > 0);
    // TODO test AST structure
}
