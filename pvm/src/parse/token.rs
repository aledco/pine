#[derive(PartialEq)]
pub(crate) struct Line {
    pub inst_token: Token,
    pub operand_tokens: Vec<Token>,
    pub line: usize
}

impl Line {
    pub fn new(inst_token: Token, operand_tokens: Vec<Token>, line: usize) -> Self {
        Self {
            inst_token,
            operand_tokens,
            line
        }
    }
}

#[derive(PartialEq)]
pub(crate) enum Token {
    Identifier(String),
    Literal(Literal),
}

#[derive(PartialEq)]
pub(crate) enum Literal {
    Integer(u64),
    Float(f64),
    Char(u8),
    String(String),
}
