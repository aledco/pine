pub struct Token {
    pub value: TokenType,
    
}

pub struct Span {
    pub line: usize,
    pub col: usize,
}

pub enum TokenType {
    Keyword(String),
    Integer(i64),
    String(String),
}