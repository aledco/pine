use ast::ast::AstType;
use ast::symbol::ScopeDepth;
use ast::{lex::lex, parse::parse};
//use ast::{lex::lex, parse::parse, traverse::traverse};
use std::path::PathBuf;
use std::str::FromStr;

extern crate tests_proc_macros;
use tests_proc_macros::make_traverse_single_file_tests;

mod common;

make_traverse_single_file_tests!();

fn test(test_base_path: &str) {
    let input = read_test_files(PathBuf::from_str(test_base_path).unwrap());
    let tokens = lex(input);
    let mut program = parse(tokens);
    // traverse(&mut program);
    // for f in &program.functions {
    //     assert_eq!(f.scope.borrow().depth, ScopeDepth::Global);
    //     match &f.ast_type {
    //         AstType::Function { identifier, .. } => match &identifier.ast_type {
    //             AstType::Identifier { name, .. } => {
    //                 let symbol = f.scope.borrow().lookup(name);
    //                 assert!(symbol.is_some())
    //             }
    //             _ => assert!(false),
    //         },
    //         _ => assert!(false),
    //     }
    // }
}

fn read_test_files(mut base_path: PathBuf) -> String {
    base_path.push("test");
    common::read_pine_input(&mut base_path).expect("pine input is mising")
}
