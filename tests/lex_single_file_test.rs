use ast::{lex::lex, token::*};
use std::path::PathBuf;
use std::str::FromStr;

extern crate tests_proc_macros;
use tests_proc_macros::make_lex_single_file_tests;

mod common;

make_lex_single_file_tests!();

fn test(test_base_path: &str) {
    let (input, token_ref) = read_test_files(PathBuf::from_str(test_base_path).unwrap());
    let actual = lex(input);
    println!("{:#?}", actual);
    match token_ref {
        Some(token_ref) => {
            let expected = parse_token_ref(token_ref);
            assert_eq!(expected, actual);
        }
        None => assert!(true),
    }
}

//         fn test(n: usize) {
//     let (input, token_ref) = read_test_files(n);
//     let actual = lex(input);
//     println!("{:#?}", actual);
//     match token_ref {
//         Some(token_ref) => {
//             let expected = parse_token_ref(token_ref);
//             assert_eq!(expected, actual);
//         }
//         None => assert!(true),
//     }
// }

fn parse_token_ref(token_ref: String) -> Vec<Token> {
    token_ref
        .lines()
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| parse_token_ref_line(line))
        .collect()
}

fn parse_token_ref_line(line: &str) -> Token {
    let parts = line.split(':').collect::<Vec<&str>>();
    if parts.len() != 2 {
        panic!("invalid line: {}", line);
    }

    let token_type = parts[0];
    let token_value = parts[1];
    let token_type = match token_type {
        // TODO finish
        "key" => TokenType::Keyword(Keyword::from_str(token_value).unwrap()),
        "id" => TokenType::Identifier(String::from(token_value)),
        "int" => TokenType::Integer(token_value.parse().unwrap()),
        "pun" => TokenType::Punctuation(Punctuation::from_str(token_value).unwrap()),
        "op" => TokenType::Operator(Operator::from_str(token_value).unwrap()),
        _ => panic!("invalid token type: {}", token_type),
    };
    Token::new(token_type, Span::default())
}

fn read_test_files(mut base_path: PathBuf) -> (String, Option<String>) {
    base_path.push("test");
    let pine_input = common::read_pine_input(&mut base_path).expect("pine input is mising");
    match common::read_token_ref(&mut base_path) {
        Ok(token_ref) => (pine_input, Some(token_ref)),
        _ => (pine_input, None),
    }
}

// fn read_test_files(i: usize) -> (String, Option<String>) {
//     let mut path = common::single_file_test_base_path(i);
//     let pine_input = common::read_pine_input(&mut path).expect("pine input is mising");
//     match common::read_token_ref(&mut path) {
//         Ok(token_ref) => (pine_input, Some(token_ref)),
//         _ => (pine_input, None),
//     }
// }
