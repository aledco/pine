use crate::token::*;

/// Processes the input into a collection of tokens.
///
/// # Arguments
/// * `input` - A string that holds a Pine program
pub fn lex(input: String) -> Vec<Token> {
    let mut scanner = Scanner::new(input);
    scanner.scan()
}

/// Represents a scanner used to lex a Pine program
struct Scanner {
    /// The input, represented as a vector of chars
    input: Vec<char>,
    /// The current index into the input
    index: usize,
    /// The current line into the input
    line: usize,
    /// The current column into the input
    col: usize,
}

impl Scanner {
    /// Creates a new scanner.
    /// 
    /// # Arguments
    /// * `input` - A string that holds a Pine program
    pub fn new(input: String) -> Self {
        Scanner {
            input: input.chars().collect(),
            index: 0,
            line: 1,
            col: 1
        }
    }
    
    /// Scans the input and produces a vector of tokens.
    /// 
    /// # Arguments
    /// * `self` - A mutable reference to the scanner.
    pub fn scan(&mut self) -> Vec<Token>  {
        let mut tokens: Vec<Token> = Vec::new();
        while !self.eof() {
            if self.is_whitespace() {
                self.advance();
            } else if self.is_comment() {
                self.skip_comment();
            } else if self.is_alphabetic() {
                let token = self.scan_identifier_or_keyword();
                tokens.push(token);
            } else if self.is_digit() {
                let token = self.scan_numeral();
                tokens.push(token);
            } else if self.is_operator() {
                let token = self.scan_operator();
                tokens.push(token);
            } else if self.is_punctuation() {
                let token = self.scan_punctuation();
                tokens.push(token);
            } else {
                // TODO need to parse punctuation
                panic!("Parse Error: Unrecognized character {} at {}", self.char(), self.point());
            }
        }

        tokens
    }

    /// Scans an identifier or keyword and returns the token.
    ///
    /// # Arguments
    /// * `self` - A mutable reference to the scanner.
    fn scan_identifier_or_keyword(&mut self) -> Token {
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

    /// Scans a numeral and returns the token.
    ///
    /// # Arguments
    /// * `self` - A mutable reference to the scanner.
    fn scan_numeral(&mut self) -> Token {
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

    /// Scans an operator and returns the token.
    ///
    /// # Arguments
    /// * `self` - A mutable reference to the scanner.
    fn scan_operator(&mut self) -> Token {
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

    /// Scans punctuation and returns the token.
    ///
    /// # Arguments
    /// * `self` - A mutable reference to the scanner.
    fn scan_punctuation(&mut self) -> Token {
        assert!(self.is_punctuation());
        let start = self.point();
        let value = self.char();
        self.advance();
        let end = self.point();
        Token::new(TokenType::Punctuation(value), Span::new(start, end))
    }

    /// Skips over single line comments in the input.
    ///
    /// # Arguments
    /// * `self` - A mutable reference to the scanner.
    fn skip_comment(&mut self) {
        assert!(self.is_comment());
        while !self.eof() && !self.is_newline() {
            self.advance();
        }
    }

    /// Advances the index into the input.
    ///
    /// # Arguments
    /// * `self` - A mutable reference to the scanner.
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

    /// Returns a value indicating whether the scanner has reached EOF.
    ///
    /// # Arguments
    /// * `self` - A reference to the scanner.
    fn eof(&self) -> bool {
        self.index >= self.input.len()
    }

    /// Returns the current char in the input.
    ///
    /// # Arguments
    /// * `self` - A reference to the scanner.
    fn char(&self) -> char {
        self.input[self.index]
    }

    /// Returns a value indicating whether the current char is a newline.
    ///
    /// # Arguments
    /// * `self` - A reference to the scanner.
    fn is_newline(&self) -> bool {
        self.char() == '\n'
    }

    /// Returns a value indicating whether the current char is whitespace.
    ///
    /// # Arguments
    /// * `self` - A reference to the scanner.
    fn is_whitespace(&self) -> bool {
        self.char().is_whitespace()
    }

    /// Returns a value indicating whether the current char is a comment.
    ///
    /// # Arguments
    /// * `self` - A reference to the scanner.
    fn is_comment(&self) -> bool {
        self.char() == '#'
    }

    /// Returns a value indicating whether the current char is alphabetic.
    ///
    /// # Arguments
    /// * `self` - A reference to the scanner.
    fn is_alphabetic(&self) -> bool {
        self.char().is_alphabetic()
    }

    /// Returns a value indicating whether the current char is a digit.
    ///
    /// # Arguments
    /// * `self` - A reference to the scanner.
    fn is_digit(&self) -> bool {
        self.char().is_digit(10)
    }

    /// Returns a value indicating whether the current char is alphabetic or numeric.
    ///
    /// # Arguments
    /// * `self` - A reference to the scanner.
    fn is_alphanumeric(&self) -> bool {
        self.is_alphabetic() || self.is_digit()
    }

    /// Returns a value indicating whether the current char is an identifier or keyword.
    ///
    /// # Arguments
    /// * `self` - A reference to the scanner.
    fn is_identifier_or_keyword(&self) -> bool {
        self.is_alphanumeric()
            || self.char() == '_'
            || self.char() == '~'
            || self.char() == '$'
            || self.char() == '@'
    }

    /// Returns a value indicating whether the current char is an operator.
    ///
    /// # Arguments
    /// * `self` - A reference to the scanner.
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

    /// Returns a value indicating whether the current char is punctuation.
    ///
    /// # Arguments
    /// * `self` - A reference to the scanner.
    fn is_punctuation(&self) -> bool {
        self.char() == '('
            || self.char() == ')'
            || self.char() == '{'
            || self.char() == '}'
            || self.char() == '['
            || self.char() == ']'
            || self.char() == ','
    }

    /// Returns the current point in the input.
    ///
    /// # Arguments
    /// * `self` - A reference to the scanner.
    fn point(&self) -> Point {
        Point::new(self.line, self.col)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    /// Returns a value indicating whether two vectors of tokens are equal.
    /// 
    /// # Arguments
    /// * `actual` - The actual vector of tokens.
    /// * `expected` - The expected vector of tokens.
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
    fn test_lex1() {
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

    /// Tests the lexer on a variable assignment.
    #[test]
    fn test_lex2() {
        let input = "
fun main() begin
    let x = 0
    if x == 0 then
        print(1)
    end

    return 0
end";

        let span = Span::new(Point::new(0, 0), Point::new(0, 0));
        let expected = vec![
            Token::new(TokenType::Keyword(Keyword::Fun), span),
            Token::new(TokenType::Identifier(String::from("main")), span),
            Token::new(TokenType::Punctuation('('), span),
            Token::new(TokenType::Punctuation(')'), span),
            Token::new(TokenType::Keyword(Keyword::Begin), span),
            Token::new(TokenType::Keyword(Keyword::Let), span),
            Token::new(TokenType::Identifier(String::from("x")), span),
            Token::new(TokenType::Operator(Operator::Assign), span),
            Token::new(TokenType::Integer(0), span),
            Token::new(TokenType::Keyword(Keyword::If), span),
            Token::new(TokenType::Identifier(String::from("x")), span),
            Token::new(TokenType::Operator(Operator::Equals), span),
            Token::new(TokenType::Integer(0), span),
            Token::new(TokenType::Keyword(Keyword::Then), span),
            Token::new(TokenType::Identifier(String::from("print")), span),
            Token::new(TokenType::Punctuation('('), span),
            Token::new(TokenType::Integer(1), span),
            Token::new(TokenType::Punctuation(')'), span),
            Token::new(TokenType::Keyword(Keyword::End), span),
            Token::new(TokenType::Keyword(Keyword::Return), span),
            Token::new(TokenType::Integer(0), span),
            Token::new(TokenType::Keyword(Keyword::End), span),
        ];

        let tokens = lex(String::from(input));
        assert!(equal(tokens, expected));
    }
}
