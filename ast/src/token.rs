pub struct Token {
    pub value: TokenType,
    pub span: Span,
}

pub struct Span {
    pub line: usize,
    pub col: usize,
}

pub enum TokenType {
    Keyword(Keyword),
    Integer(i64),
    String(String),
}

pub enum Keyword {
    If,
    For,
}

impl Token {
    pub fn new(value: TokenType, span: Span) -> Self {
        Token { value, span }
    }
}