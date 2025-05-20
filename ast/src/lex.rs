use crate::operator::Operator;
use crate::token::*;
use std::cmp::{max, min};
use std::str::FromStr;
use crate::error::{ParseError, ParseResult};

/// Processes the input into a collection of tokens.
///
/// # Arguments
/// * `input` - A string that holds a Pine program
pub fn lex(input: String) -> ParseResult<Vec<Token>> {
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
            col: 1,
        }
    }

    /// Scans the input and produces a vector of tokens.
    ///
    /// # Arguments
    /// * `self` - A mutable reference to the scanner.
    pub fn scan(&mut self) -> ParseResult<Vec<Token>> {
        let mut tokens: Vec<Token> = Vec::new();
        while !self.eof() {
            if self.is_whitespace() {
                self.advance();
            } else if self.is_comment() {
                self.skip_comment();
            } else {
                let token = if self.is_alphabetic() {
                    self.scan_identifier_or_keyword_or_operator()
                } else if self.is_digit() {
                    self.scan_numeral()
                } else if self.is_punctuation() || self.is_operator() {
                    self.scan_punctuation_or_operator()
                } else {
                    Err(ParseError::new("unrecognized token", Span::new(self.point(), self.point())))
                };
                
                tokens.push(token?);
            }
        }

        Ok(tokens)
    }

    /// Scans an identifier, keyword, or operator and returns the token.
    ///
    /// # Arguments
    /// * `self` - A mutable reference to the scanner.
    fn scan_identifier_or_keyword_or_operator(&mut self) ->ParseResult<Token> {
        assert!(self.is_alphabetic());
        let start = self.point();
        let mut value = String::new();
        while !self.eof() && self.is_identifier_or_keyword() {
            value.push(self.char());
            self.advance();
        }

        let end = self.point();
        let token_type = if let Ok(keyword) = Keyword::from_str(value.as_str()) {
            TokenType::Keyword(keyword)
        } else if let Ok(operator) = Operator::from_str(value.as_str()) {
            TokenType::Operator(operator)
        } else {
            TokenType::Identifier(value)
        };
        Ok(Token::new(token_type, Span::new(start, end)))
    }

    /// Scans a numeral and returns the token.
    ///
    /// # Arguments
    /// * `self` - A mutable reference to the scanner.
    fn scan_numeral(&mut self) -> ParseResult<Token> {
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
                return Err(ParseError::new("invalid numeral", Span::new(start, start)));
            }

            while !self.eof() && self.is_digit() {
                value.push(self.char());
                self.advance();
            }

            let end = self.point();
            let float: f64 = value.parse().unwrap();
            Ok(Token::new(TokenType::Float(float), Span::new(start, end)))
        } else {
            let end = self.point();
            let integer: i64 = value.parse().unwrap();
            Ok(Token::new(TokenType::Integer(integer), Span::new(start, end)))
        }
    }

    /// Scans punctuation or operator and returns the token.
    ///
    /// # Arguments
    /// * `self` - A mutable reference to the scanner.
    fn scan_punctuation_or_operator(&mut self) -> ParseResult<Token> {
        assert!(self.is_punctuation() || self.is_operator());
        let start = self.point();
        let mut length = max(Punctuation::get_max_length(), Operator::max_length());
        length = min(length, self.input.len() - self.index); // TODO check if this is right
        while length > 0 {
            let slice = self.slice(length);
            if let Ok(punctuation) = Punctuation::from_str(slice.as_str()) {
                self.advance_n(length);
                let end = self.point();
                return Ok(Token::new(
                    TokenType::Punctuation(punctuation),
                    Span::new(start, end),
                ));
            } else if let Ok(operator) = Operator::from_str(slice.as_str()) {
                self.advance_n(length);
                let end = self.point();
                return Ok(Token::new(
                    TokenType::Operator(operator),
                    Span::new(start, end),
                ));
            } else {
                length -= 1;
            }
        }

        Err(ParseError::new("invalid token", Span::new(start, start)))
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

    /// Advances the index into the input.
    ///
    /// # Arguments
    /// * `self` - A mutable reference to the scanner.
    /// * `n` - The number of advances to make.
    fn advance_n(&mut self, n: usize) {
        assert!(!self.eof());
        for _ in 0..n {
            self.advance();
        }
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

    /// Returns a slice of the input.
    ///
    /// # Arguments
    /// * `self` - A reference to the scanner.
    /// * `length` - The length of the slice.
    fn slice(&self, length: usize) -> String {
        let slice: String = self.input[self.index..self.index + length].iter().collect();
        slice
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
        Operator::all_values()
            .into_iter()
            .filter(|v| v.contains(self.char()))
            .count()
            > 0
    }

    /// Returns a value indicating whether the current char is punctuation.
    ///
    /// # Arguments
    /// * `self` - A reference to the scanner.
    fn is_punctuation(&self) -> bool {
        Punctuation::get_all_values()
            .into_iter()
            .filter(|v| v.contains(self.char()))
            .count()
            > 0
    }

    /// Returns the current point in the input.
    ///
    /// # Arguments
    /// * `self` - A reference to the scanner.
    fn point(&self) -> Point {
        Point::new(self.line, self.col)
    }
}
