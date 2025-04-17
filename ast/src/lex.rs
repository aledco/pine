use crate::token::Token;

pub fn lex(input: String) -> Vec<Token> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = lex(String::from(""));
        assert_eq!(result.len(), 0);
    }
}