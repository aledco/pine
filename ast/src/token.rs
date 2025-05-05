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
    pub fn default() -> Self {
        Self {
            token_type: TokenType::Keyword(Keyword::Fun),
            span: Span::default(),
        }
    }

    /// Creates a new token from the token type and span.
    ///
    /// # Arguments
    /// * `value` - The value of the token.
    /// * `span` - The span of the token.
    pub fn new(value: TokenType, span: Span) -> Self {
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenTypeMatch {
    Identifier,
    Integer,
    Float,
    String,
    Operator,
}

pub trait TokenMatch {
    fn matches(&self, token_type: &TokenType) -> bool;
}

impl TokenMatch for TokenType {
    fn matches(&self, token_type: &TokenType) -> bool {
        self == token_type
    }
}

impl TokenMatch for TokenTypeMatch {
    fn matches(&self, token_type: &TokenType) -> bool {
        match token_type {
            TokenType::Identifier(_) => self == &TokenTypeMatch::Identifier,
            TokenType::Integer(_) => self == &TokenTypeMatch::Integer,
            TokenType::Float(_) => self == &TokenTypeMatch::Float,
            TokenType::String(_) => self == &TokenTypeMatch::String,
            TokenType::Operator(_) => self == &TokenTypeMatch::Operator,
            _ => false,
        }
    }
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
    #[strum(serialize = "set")]
    Set,
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
    #[strum(serialize = "void")]
    Void,
}

impl TokenMatch for Keyword {
    fn matches(&self, token_type: &TokenType) -> bool {
        match token_type {
            TokenType::Keyword(k) => self == k,
            _ => false,
        }
    }
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

impl TokenMatch for Punctuation {
    fn matches(&self, token_type: &TokenType) -> bool {
        match token_type {
            TokenType::Punctuation(p) => self == p,
            _ => false,
        }
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
    pub fn all_values() -> Vec<String> {
        Self::iter()
            .filter(|p| p.get_str("Value").is_some())
            .map(|p| p.get_str("Value").unwrap())
            .map(|s| String::from(s))
            .collect()
    }

    pub fn all_unary_ops() -> Vec<Self> {
        Self::iter().filter(|op| op.is_unary()).collect()
    }

    pub fn all_binary_ops() -> Vec<Self> {
        Self::iter().filter(|op| op.is_binary()).collect()
    }

    pub fn binary_ops_by_precedence(precedence: i32) -> Vec<Self> {
        Self::all_binary_ops()
            .into_iter()
            .filter(|op| op.precedence() == precedence)
            .collect()
    }

    pub fn max_length() -> usize {
        Self::all_values()
            .into_iter()
            .max_by(|a, b| a.len().cmp(&b.len()))
            .unwrap()
            .len()
    }

    pub fn max_precedence() -> i32 {
        Self::iter()
            .max_by(|a, b| a.precedence().cmp(&b.precedence()))
            .unwrap()
            .precedence()
    }

    pub fn min_precedence() -> i32 {
        Self::iter()
            .min_by(|a, b| a.precedence().cmp(&b.precedence()))
            .unwrap()
            .precedence()
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
}

impl TokenMatch for Operator {
    fn matches(&self, token_type: &TokenType) -> bool {
        match token_type {
            TokenType::Operator(o) => self == o,
            _ => false,
        }
    }
}

/// Represents a point in the input
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
    pub line: usize,
    pub col: usize,
}

impl Point {
    pub fn default() -> Self {
        Self { line: 0, col: 0 }
    }

    pub fn new(line: usize, col: usize) -> Self {
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
    pub fn default() -> Self {
        Self {
            start: Point::default(),
            end: Point::default(),
        }
    }

    pub fn new(start: Point, end: Point) -> Self {
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
        Span::new(
            Point::new(self.start.line, self.start.col),
            Point::new(rhs.end.line, rhs.end.col),
        )
    }
}
