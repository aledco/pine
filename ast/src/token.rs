use std::fmt;

pub struct Token {
    pub value: TokenType,
    pub span: Span,
}

impl Token {
    pub fn new(value: TokenType, span: Span) -> Self {
        Token { value, span }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

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

#[derive(PartialEq)]
pub enum Keyword {
    Fun,
    Begin,
    End,
    If,
    Else,
    For,
    While,
    Return,
}

impl Keyword {
    pub fn from(value: &str) -> Option<Keyword> {
        match value {
            "fun" => Some(Keyword::Fun),
            "begin" => Some(Keyword::Begin),
            "end" => Some(Keyword::End),
            "if" => Some(Keyword::If),
            "else" => Some(Keyword::Else),
            "for" => Some(Keyword::For),
            "while" => Some(Keyword::While),
            "return" => Some(Keyword::Return),
            _ => None,
        }
    }
}

#[derive(PartialEq)]
pub enum Operator {
    Assign, // =
    Equals, // ==
    NotEquals, // !=
    GreaterThan, // >
    LessThan, // <
    GreaterThanOrEqual, // >=
    LessThanOrEqual, // <=
}

impl Operator {
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
