use ast::*;
use std::str::FromStr;

pub(crate) fn parse_token_ref(token_ref: String) -> Vec<Token> {
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
        "flt" => TokenType::Float(token_value.parse().unwrap()),
        "pun" => TokenType::Punctuation(Punctuation::from_str(token_value).unwrap()),
        "op" => TokenType::Operator(Operator::from_str(token_value).unwrap()),
        _ => panic!("invalid token type: {}", token_type),
    };
    Token::new(token_type, Span::default())
}
