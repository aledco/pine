use crate::parse::token::*;

pub(crate) fn lex(input: &str) -> Result<Vec<Line>, String> {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| lex_line(i, line))
        .filter(|line| line.is_some())
        .map(|line| line.unwrap())
        .collect()
}

fn lex_line(i: usize, line: &str) -> Option<Result<Line, String>> {
    let parts = line.split_whitespace().collect::<Vec<_>>();
    if parts.is_empty() {
        return None;
    }

    let inst = parts.first().unwrap();
    if inst.starts_with('#') {
        return None;
    } else if inst.is_empty() {
        return Some(Err(format!("Error at line {}: empty instruction", i)));
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
        return Some(Err(format!("Error at line {}: invalid operand", i)));
    }
    
    let operand_tokens = operand_tokens.into_iter().map(|r| r.unwrap()).collect();
    Some(Ok(Line::new(inst_token, operand_tokens, i)))
}

fn lex_operand(mut value: &str) -> Option<Result<Token, ()>> {
    if value.starts_with('#') {
        return None;
    }
    
    if value.ends_with(',') {
        value = &value[0..value.len()-1];
    }
    
    if value.chars().nth(0).unwrap().is_alphabetic() {
        if value.chars().all(|c| c.is_alphanumeric() || c == '_') { // TODO add function to test if char is valid identifier
            Some(Ok(Token::Identifier(value.to_string())))
        } else {
            Some(Err(()))
        }
    } else if value.chars().nth(0).unwrap().is_numeric() {
        if let Ok(v) = value.parse::<i64>() {
            Some(Ok(Token::Literal(Literal::Integer(v))))
        } else if let Ok(v) = value.parse::<f64>() {
            Some(Ok(Token::Literal(Literal::Float(v))))
        } else {
            Some(Err(()))
        }
    } else {
        return Some(Err(()));
    }
}
