use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use ast::{lex::lex, token::*};

extern crate tests_proc_macros;
use tests_proc_macros::make_lex_single_file_tests;

make_lex_single_file_tests!();

fn test(n: usize) {
    let (input, token_ref) = read_test_files(n);
    let actual = lex(input);
    match token_ref {
        Some(token_ref) => {
            let expected = parse_token_ref(token_ref);
            assert_eq!(expected, actual);
        },
        None => assert!(true)
    }
}

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
    let token_type = match token_type { // TODO finish
        "key" => TokenType::Keyword(Keyword::from_str(token_value).unwrap()),
        "id" => TokenType::Identifier(String::from(token_value)),
        "int" => TokenType::Integer(token_value.parse().unwrap()),
        "pun" => TokenType::Punctuation(Punctuation::from_str(token_value).unwrap()),
        "op" => TokenType::Operator(Operator::from_str(token_value).unwrap()),
        _ => panic!("invalid token type: {}", token_type),
    };
    Token::from(token_type, Span::new())
}

fn read_test_files(n: usize) -> (String, Option<String>) {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(format!("resources/tests/single_file/test{}/test", n));
    path.set_extension("p");
    let pine_input = fs::read_to_string(path.as_path()).unwrap();

    path.set_extension("tok");
    match fs::read_to_string(path.as_path()) {
        Ok(token_ref) => (pine_input, Some(token_ref)),
        _ => (pine_input, None)
    }
}
