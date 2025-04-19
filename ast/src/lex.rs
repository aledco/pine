use crate::token::*;

pub fn lex(input: String) -> Vec<Token> {
    let mut scanner = Scanner::new(input);
    scanner.scan()
}

struct Scanner {
    input: Vec<char>,
    index: usize,
    line: usize,
    col: usize,
}

impl Scanner {
    pub fn new(input: String) -> Self {
        Scanner {
            input: input.chars().collect(),
            index: 0,
            line: 1,
            col: 1
        }
    }

    pub fn scan(&mut self, ) -> Vec<Token>  {
        let mut tokens: Vec<Token> = Vec::new();
        while !self.eof() {
            if self.is_whitespace() {
                self.advance();
            } else if self.is_comment() {
                self.skip_comment();
            } else if self.is_alphabetic() {
                let token = self.lex_identifier_or_keyword();
                tokens.push(token);
            } else if self.is_digit() {
                let token = self.lex_numeral();
                tokens.push(token);
            } else if self.is_operator() {
                let token = self.lex_operator();
                tokens.push(token);
            } else if self.is_punctuation() {
                let token = self.lex_punctuation();
                tokens.push(token);
            } else {
                // TODO need to parse punctuation
                panic!("Parse Error: Unrecognized character {} at {}", self.char(), self.point());
            }
        }

        tokens
    }
    
    fn lex_identifier_or_keyword(&mut self) -> Token {
        assert!(self.is_alphabetic());
        let start = self.point();
        let mut value = String::new();
        while !self.eof() && self.is_identifier_or_keyword() {
            value.push(self.char());
            self.advance();
        }

        let end = self.point();
        let token_type =
            if let Some(keyword) = Keyword::from(value.as_str()) {
                TokenType::Keyword(keyword)
            } else {
                TokenType::Identifier(value)
            };

        Token::new(token_type, Span::new(start, end))
    }

    fn lex_numeral(&mut self) -> Token {
        assert!(self.is_digit());
        let start = self.point();
        let mut value = String::new();
        while !self.eof() && self.is_digit() {
            value.push(self.char());
            self.advance();
        }

        if self.char() == '.' {
            value.push(self.char());
            self.advance();
            if !self.is_digit() {
                panic!("Parse Error: Invalid numeral at {}", start);
            }

            while !self.eof() && self.is_digit() {
                value.push(self.char());
                self.advance();
            }

            let end = self.point();
            let float: f64 = value.parse().unwrap();
            Token::new(TokenType::Float(float), Span::new(start, end))
        } else {
            let end = self.point();
            let integer: i64 = value.parse().unwrap();
            Token::new(TokenType::Integer(integer), Span::new(start, end))
        }
    }
    
    fn lex_operator(&mut self) -> Token {
        assert!(self.is_operator());
        let start = self.point();
        let mut value = String::new();
        while !self.eof() && self.is_operator() {
            value.push(self.char());
            self.advance();
        }
        
        if let Some(operator) = Operator::from(value.as_str()) {
            let end = self.point();
            Token::new(TokenType::Operator(operator), Span::new(start, end))
        } else {
            panic!("Parse Error: Invalid operator {} at {}", value, start);
        }
    }
        
    fn lex_punctuation(&mut self) -> Token {
        assert!(self.is_punctuation());
        let start = self.point();
        let value = self.char();
        self.advance();
        let end = self.point();
        Token::new(TokenType::Punctuation(value), Span::new(start, end))
    }
    fn skip_comment(&mut self) {
        assert!(self.is_comment());
        while !self.eof() && !self.is_newline() {
            self.advance();
        }
    }

    fn advance(&mut self) {
        assert!(!self.eof());
        if self.is_newline() {
            self.col = 1;
            self.line += 1;
        } else {
            self.col += 1;
        }

        self.index += 1;
    }

    fn eof(&self) -> bool {
        self.index >= self.input.len()
    }

    fn char(&self) -> char {
        self.input[self.index]
    }

    fn is_newline(&self) -> bool {
        self.char() == '\n'
    }
    
    fn is_whitespace(&self) -> bool {
        self.char().is_whitespace()
    }
    
    fn is_comment(&self) -> bool {
        self.char() == '#'
    }
    
    fn is_alphabetic(&self) -> bool {
        self.char().is_alphabetic()
    }
    
    fn is_digit(&self) -> bool {
        self.char().is_digit(10)
    }
    
    fn is_alphanumeric(&self) -> bool {
        self.is_alphabetic() || self.is_digit()
    }

    fn is_identifier_or_keyword(&self) -> bool {
        self.is_alphanumeric()
            || self.char() == '_'
            || self.char() == '$'
            || self.char() == '@'
    }
    
    fn is_operator(&self) -> bool {
        self.char() == '='
            || self.char() == '!'
            || self.char() == '<'
            || self.char() == '>'
            || self.char() == '+'
            || self.char() == '-'
            || self.char() == '*'
            || self.char() == '/'
            || self.char() == '%'
    }
    
    fn is_punctuation(&self) -> bool {
        self.char() == '('
            || self.char() == ')'
            || self.char() == '{'
            || self.char() == '}'
            || self.char() == '['
            || self.char() == ']'
            || self.char() == ','
    }

    fn point(&self) -> Point {
        Point::new(self.line, self.col)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn equal(actual: Vec<Token>, expected: Vec<Token>) -> bool {
        if actual.len() != expected.len() {
            return false;
        }
        
        for (actual, expected) in actual.into_iter().zip(expected.into_iter()) {
            if actual != expected {
                return false;
            }
        }
        
        true
    }
    
    #[test]
    fn lex_function() {
        let input = "
fun main() begin
    return 0
end";
        
        let span = Span::new(Point::new(0, 0), Point::new(0, 0));
        let expected = vec![
            Token::new(TokenType::Keyword(Keyword::Fun), span),
            Token::new(TokenType::Identifier(String::from("main")), span),
            Token::new(TokenType::Punctuation('('), span),
            Token::new(TokenType::Punctuation(')'), span),
            Token::new(TokenType::Keyword(Keyword::Begin), span),
            Token::new(TokenType::Keyword(Keyword::Return), span),
            Token::new(TokenType::Integer(0), span),
            Token::new(TokenType::Keyword(Keyword::End), span),
        ];
        
        let tokens = lex(String::from(input));
        assert!(equal(tokens, expected));
    }
}