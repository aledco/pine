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
