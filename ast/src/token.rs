use std::fmt;

/// Represents a token
pub struct Token {
    /// The value of the token
    pub value: TokenType,
    /// The span of the token in the input
    pub span: Span,
}

impl Token {
    /// Creates a new token.
    /// 
    /// # Arguments
    /// * `value` - The value of the token.
    /// * `span` - The span of the token.
    pub fn new(value: TokenType, span: Span) -> Self {
        Token { value, span }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

/// Represents the token type
#[derive(PartialEq)]
pub enum TokenType {
    Keyword(Keyword),
    Identifier(String),
    Integer(i64),
    Float(f64),
    String(String),
    Punctuation(char),
    Operator(Operator),
}

/// Represents a Pine keyword
#[derive(PartialEq)]
pub enum Keyword {
    Fun,
    Begin,
    End,
    Let,
    If,
    Then,
    Else,
    For,
    While,
    Do,
    Return,
}

impl Keyword {
    /// Creates a keyword from a string slice.
    /// 
    /// # Arguments
    /// * `value` - The string slice.
    pub fn from(value: &str) -> Option<Keyword> {
        match value {
            "fun" => Some(Keyword::Fun),
            "begin" => Some(Keyword::Begin),
            "end" => Some(Keyword::End),
            "let" => Some(Keyword::Let),
            "if" => Some(Keyword::If),
            "then" => Some(Keyword::Then),
            "else" => Some(Keyword::Else),
            "for" => Some(Keyword::For),
            "while" => Some(Keyword::While),
            "do" => Some(Keyword::Do),
            "return" => Some(Keyword::Return),
            _ => None,
        }
    }
}

/// Represents a Pine operator
#[derive(PartialEq)]
pub enum Operator {
    /// The `=` operator
    Assign,
    /// The `==` operator
    Equals,
    /// The `!=` operator
    NotEquals,
    /// The `>` operator
    GreaterThan,
    /// The `<` operator
    LessThan,
    /// The `>=` operator
    GreaterThanOrEqual,
    /// The `<=` operator
    LessThanOrEqual,
}

impl Operator {
    /// Creates an operator from a string slice.
    ///
    /// # Arguments
    /// * `value` - The string slice.
    pub fn from(value: &str) -> Option<Operator> {
        match value {
            "=" => Some(Operator::Assign),
            "==" => Some(Operator::Equals),
            "!=" => Some(Operator::NotEquals),
            ">" => Some(Operator::GreaterThan),
            "<" => Some(Operator::LessThan),
            ">=" => Some(Operator::GreaterThanOrEqual),
            "<=" => Some(Operator::LessThanOrEqual),
            _ => None,
        }
    }
}

/// Represents a point in the input
#[derive(PartialEq, Copy, Clone)]
pub struct Point {
    pub line: usize,
    pub col: usize,
}

impl Point {
    pub fn new(line: usize, col: usize) -> Self {
        Point{
            line,
            col
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.col)
    }
}

/// Represents a span in the input
#[derive(PartialEq, Copy, Clone)]
pub struct Span {
    pub start: Point,
    pub end: Point,
}

impl Span {
    pub fn new(start: Point, end: Point) -> Self {
        Span {
            start,
            end
        }
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.start)
    }
}
