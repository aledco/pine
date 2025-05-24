use crate::error::Error;
use crate::parse::error::ParseError;
use crate::parse::token::*;

/// Scans the input and produces a sequence of lines.
pub(crate) fn lex(input: String) -> Result<Vec<Line>, Error> {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| lex_line(i, line))
        .filter(|line| line.is_some())
        .map(|line| line.unwrap())
        .collect()
}

/// Scans a line.
fn lex_line(i: usize, line: &str) -> Option<Result<Line, Error>> {
    let parts = line.split_whitespace().collect::<Vec<_>>();
    let inst = match parts.first() {
        Some(part) => *part,
        None => return None,
    };
    
    if inst.starts_with('#') {
        return None;
    }
    
    let inst_token = Token::Identifier(inst.to_string());
    let operand_tokens: Vec<Result<Token, ()>> = parts
        .iter()
        .skip(1)
        .map(|p| lex_operand(p))
        .take_while(|o| o.is_some())
        .map(|o| o.unwrap())
        .collect();
    
    if operand_tokens.iter().any(|r| r.is_err()) {
        return Some(Err(ParseError::invalid_operand(i+1)));
    }
    
    let operand_tokens = operand_tokens.into_iter().map(|r| r.unwrap()).collect();
    Some(Ok(Line::new(inst_token, operand_tokens, i+1)))
}

/// Scans an operand.
fn lex_operand(mut value: &str) -> Option<Result<Token, ()>> {
    if value.starts_with('#') {
        return None;
    }
    
    if value.ends_with(',') {
        value = &value[0..value.len()-1];
    }
    
    if value.chars().nth(0).unwrap().is_alphabetic() {
        Some(lex_identifier(value))
    } else if value.chars().nth(0).unwrap().is_numeric() {
        Some(lex_number(value))
    } else if value.chars().nth(0).unwrap() == '\'' {
        Some(lex_char(value))
    } else {
        Some(Err(()))
    }
}

/// Scans an identifer.
fn lex_identifier(value: &str) -> Result<Token, ()> {
    if value.chars().all(|c| is_valid_identifier_char(c)) {
        Ok(Token::Identifier(value.to_string()))
    } else {
        Err(())
    }
}

/// Determines if a character is a valid identifier character.
fn is_valid_identifier_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '.' || c == '$' || c == ':'
}

/// Scans a number.
fn lex_number(mut value: &str) -> Result<Token, ()> {
    if value.starts_with("0x") {
        value = value.trim_start_matches("0x");
        match u64::from_str_radix(value, 16) {
            Ok(n) => Ok(Token::Literal(Literal::Integer(n))),
            Err(_) => Err(()),
        }
    } else if value.starts_with("0b") {
        value = value.trim_start_matches("0b");
        match u64::from_str_radix(value, 2) {
            Ok(n) => Ok(Token::Literal(Literal::Integer(n))),
            Err(_) => Err(()),
        }
    } else if value.chars().last().unwrap() == 'f' {
        value = &value[0..value.len()-1];
        match value.parse::<f64>() {
            Ok(f) => Ok(Token::Literal(Literal::Float(f))),
            Err(_) => Err(()),
        }
    } else if let Ok(v) = value.parse::<u64>() {
        Ok(Token::Literal(Literal::Integer(v)))
    } else if let Ok(v) = value.parse::<f64>() {
        Ok(Token::Literal(Literal::Float(v)))
    } else {
        Err(())
    }
}

/// Scans a char.
fn lex_char(value: &str)  -> Result<Token, ()> {
    let c = value.replace('\'', "");
    if c.len() != 1 { // only support one character ascii chars. escaped chars like '\n' must be specified with ascii code
        Err(())
    } else {
        let b = c.as_bytes()[0];
        Ok(Token::Literal(Literal::Char(b)))
    }
}
