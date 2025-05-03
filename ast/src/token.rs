use std::fmt;
use std::ops;
use strum::{EnumProperty, IntoEnumIterator};
use strum_macros::{EnumIter, EnumProperty, EnumString};

/// Represents a token
#[derive(Debug, Clone)]
pub struct Token {
    /// The value of the token
    pub token_type: TokenType,
    /// The span of the token in the input
    pub span: Span,
}

impl Token {
    /// Creates a new token.
    pub fn new() -> Self {
        Self {
            token_type: TokenType::Keyword(Keyword::Fun),
            span: Span::new(),
        }
    }

    /// Creates a new token from the token type and span.
    ///
    /// # Arguments
    /// * `value` - The value of the token.
    /// * `span` - The span of the token.
    pub fn from(value: TokenType, span: Span) -> Self {
        Self {
            token_type: value,
            span,
        }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.token_type == other.token_type
    }
}

/// Represents the token type
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Keyword(Keyword),
    Identifier(String),
    Integer(i64),
    Float(f64),
    String(String),
    Punctuation(Punctuation),
    Operator(Operator),
}

/// Represents a Pine keyword
#[derive(Debug, PartialEq, Copy, Clone, EnumString)]
pub enum Keyword {
    #[strum(serialize = "fun")]
    Fun,
    #[strum(serialize = "begin")]
    Begin,
    #[strum(serialize = "end")]
    End,
    #[strum(serialize = "let")]
    Let,
    #[strum(serialize = "if")]
    If,
    #[strum(serialize = "then")]
    Then,
    #[strum(serialize = "else")]
    Else,
    #[strum(serialize = "for")]
    For,
    #[strum(serialize = "while")]
    While,
    #[strum(serialize = "do")]
    Do,
    #[strum(serialize = "return")]
    Return,
    #[strum(serialize = "int")]
    Int,
    #[strum(serialize = "float")]
    Float,
    #[strum(serialize = "string")]
    String,
}

/// Represents punctuation in a Pine program
#[derive(Debug, PartialEq, Copy, Clone, EnumIter, EnumString, EnumProperty)]
pub enum Punctuation {
    #[strum(serialize = "(", props(Value = "("))]
    OpenParen,
    #[strum(serialize = ")", props(Value = ")"))]
    CloseParen,
    #[strum(serialize = "[", props(Value = "["))]
    OpenBracket,
    #[strum(serialize = "]", props(Value = "]"))]
    CloseBracket,
    #[strum(serialize = "{", props(Value = "{"))]
    OpenBrace,
    #[strum(serialize = "}", props(Value = "}"))]
    CloseBrace,
    #[strum(serialize = ",", props(Value = ","))]
    Comma,
    #[strum(serialize = ":", props(Value = ":"))]
    Colon,
    #[strum(serialize = "->", props(Value = "->"))]
    Arrow,
    #[strum(serialize = "=", props(Value = "="))]
    EqualSign,
}

impl Punctuation {
    pub fn get_all_values() -> Vec<String> {
        Self::iter()
            .filter(|p| p.get_str("Value").is_some())
            .map(|p| p.get_str("Value").unwrap())
            .map(|s| String::from(s))
            .collect()
    }

    pub fn get_max_length() -> usize {
        Self::get_all_values()
            .into_iter()
            .max_by(|a, b| a.len().cmp(&b.len()))
            .unwrap()
            .len()
    }
}

/// Represents a Pine operator
#[derive(Debug, PartialEq, Copy, Clone, EnumIter, EnumString, EnumProperty)]
pub enum Operator {
    #[strum(
        serialize = "==",
        props(Value = "==", IsUnary = false, IsBinary = true)
    )]
    Equals,
    #[strum(
        serialize = "!=",
        props(Value = "!=", IsUnary = false, IsBinary = true)
    )]
    NotEquals,
    #[strum(serialize = ">", props(Value = ">", IsUnary = false, IsBinary = true))]
    GreaterThan,
    #[strum(serialize = "<", props(Value = "<", IsUnary = false, IsBinary = true))]
    LessThan,
    #[strum(
        serialize = ">=",
        props(Value = ">=", IsUnary = false, IsBinary = true)
    )]
    GreaterThanOrEqual,
    #[strum(
        serialize = "<=",
        props(Value = "<=", IsUnary = false, IsBinary = true)
    )]
    LessThanOrEqual,
    #[strum(
        serialize = "and",
        props(Value = "and", IsUnary = false, IsBinary = true)
    )]
    And,
    #[strum(
        serialize = "or",
        props(Value = "or", IsUnary = false, IsBinary = true)
    )]
    Or,
    #[strum(
        serialize = "not",
        props(Value = "not", IsUnary = true, IsBinary = false)
    )]
    Not,
    #[strum(serialize = "+", props(Value = "+", IsUnary = false, IsBinary = true))]
    Add,
    #[strum(serialize = "-", props(Value = "-", IsUnary = true, IsBinary = true))]
    Subtract,
    #[strum(serialize = "*", props(Value = "*", IsUnary = false, IsBinary = true))]
    Multiply,
    #[strum(serialize = "/", props(Value = "/", IsUnary = false, IsBinary = true))]
    Divide,
    #[strum(
        serialize = "**",
        props(Value = "**", IsUnary = false, IsBinary = true)
    )]
    Power,
    #[strum(serialize = "%", props(Value = "%", IsUnary = false, IsBinary = true))]
    Modulo,
}

impl Operator {
    pub fn get_all_values() -> Vec<String> {
        Self::iter()
            .filter(|p| p.get_str("Value").is_some())
            .map(|p| p.get_str("Value").unwrap())
            .map(|s| String::from(s))
            .collect()
    }

    pub fn get_max_length() -> usize {
        Self::get_all_values()
            .into_iter()
            .max_by(|a, b| a.len().cmp(&b.len()))
            .unwrap()
            .len()
    }

    pub fn precedence(&self) -> i32 {
        match self {
            Operator::Equals => 4,
            Operator::NotEquals => 4,
            Operator::GreaterThan => 4,
            Operator::LessThan => 4,
            Operator::GreaterThanOrEqual => 4,
            Operator::LessThanOrEqual => 4,
            Operator::And => 6,
            Operator::Or => 7,
            Operator::Not => 5,
            Operator::Add => 3,
            Operator::Subtract => 3,
            Operator::Multiply => 2,
            Operator::Divide => 2,
            Operator::Power => 1,
            Operator::Modulo => 2,
        }
    }

    pub fn is_unary(&self) -> bool {
        self.get_bool("IsUnary").unwrap()
    }

    pub fn is_binary(&self) -> bool {
        self.get_bool("IsBinary").unwrap()
    }

    pub fn get_all_unary_ops() -> Vec<Self> {
        Operator::iter().filter(|op| op.is_unary()).collect()
    }

    pub fn get_all_binary_ops() -> Vec<Self> {
        Operator::iter().filter(|op| op.is_binary()).collect()
    }

    pub fn get_binary_ops_by_precedence(precedence: i32) -> Vec<Self> {
        Self::get_all_binary_ops()
            .into_iter()
            .filter(|op| op.precedence() == precedence)
            .collect()
    }
}

/// Represents a point in the input
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
    pub line: usize,
    pub col: usize,
}

impl Point {
    pub fn new() -> Self {
        Self { line: 0, col: 0 }
    }

    pub fn from(line: usize, col: usize) -> Self {
        Self { line, col }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.col)
    }
}

/// Represents a span in the input
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Span {
    pub start: Point,
    pub end: Point,
}

impl Span {
    pub fn new() -> Self {
        Self { start: Point::new(), end: Point::new() }
    }

    pub fn from(start: Point, end: Point) -> Self {
        Span { start, end }
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.start)
    }
}

impl ops::Add<Span> for Span {
    type Output = Span;

    fn add(self, rhs: Span) -> Span {
        Span::from(
            Point::from(self.start.line, self.start.col),
            Point::from(rhs.end.line, rhs.end.col),
        )
    }
}
