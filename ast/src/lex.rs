use crate::token::Token;

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
            if self.is_newline() {
                self.skip_newline();
            } else if self.is_whitespace() {
                self.skip_whitespace();
            } else if self.is_comment() {
                self.skip_comment();
            } else if self.is_alphabetic() {
                // check for keywords and identifiers
            } else if self.is_digit() {
                // check for numbers
            } else {
                // TODO need to parse punctuation
                panic!("Parse Error: Unrecognized character {}", self.char())
            }
        }

        tokens
    }
    
    fn lex_identifier_or_keyword(&mut self) -> Token {
        assert!(self.is_alphabetic());
        let mut value = String::new();
        while !self.eof() && self.is_alphanumeric() {
            value.push(self.char());
        }
        
        Token::new(value, span)
    }
    
    fn skip_comment(&mut self) {
        while !self.eof() && !self.is_newline() {
            self.index += 1;
        }
        
        if !self.eof() {
            assert!(self.is_newline());
            self.skip_newline()
        }
    }

    fn skip_whitespace(&mut self) {
        self.col += 1;
        self.index += 1;
    }

    fn skip_newline(&mut self) {
        self.line += 1;
        self.col = 1;
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