use crate::token::*;

/// The type of Pine construct.
#[derive(Clone, Debug, PartialEq)]
pub enum PineType {
    Integer,
    Float,
    String,
    List(Box<PineType>),
    Function {
        params: Vec<PineType>,
        ret: Box<PineType>,
    },
    Void,
    Unknown,
}

pub enum AstType {
    Function {
        identifier: Box<AstNode>,
        params: Vec<AstNode>,
        body: Box<AstNode>,
    },
    Block(Vec<AstNode>),
    Let {
        identifier: Box<AstNode>,
        expression: Box<AstNode>,
    },
    BinaryExpression {
        lhs: Box<AstNode>,
        op: Operator,
        rhs: Box<AstNode>,
    },
    UnaryExpression {
        op: Operator,
        expr: Box<AstNode>,
    },
    IdentifierExpression(Box<AstNode>),
    IntegerExpression(i64),
    FloatExpression(f64),
    StringExpression(String),
    Identifier(String),
    Dummy,
}

pub struct AstNode {
    pub ast_type: AstType,
    pub pine_type: PineType,
    pub span: Span,
}

impl AstNode {
    pub fn new_function(
        identifier: Box<AstNode>,
        params: Vec<AstNode>,
        body: Box<AstNode>,
        pine_type: PineType,
        span: Span,
    ) -> Self {
        Self {
            ast_type: AstType::Function {
                identifier,
                params,
                body,
            },
            pine_type,
            span,
        }
    }

    pub fn new_block(statements: Vec<AstNode>, span: Span) -> Self {
        Self {
            ast_type: AstType::Block(statements),
            pine_type: PineType::Void,
            span,
        }
    }
    
    pub fn new_let(identifier: Box<AstNode>, expression: Box<AstNode>, span: Span) -> Self {
        Self {
            ast_type: AstType::Let {
                identifier,
                expression
            },
            pine_type: PineType::Void,
            span,
        }
    }

    pub fn new_binary_expression(lhs: Box<AstNode>, op: Operator, rhs: Box<AstNode>, span: Span) -> Self {
        Self {
            ast_type: AstType::BinaryExpression {
                lhs,
                op,
                rhs
            },
            pine_type: PineType::Unknown,
            span
        }
    }

    pub fn new_unary_expression(op: Operator, expr: Box<AstNode>, span: Span) -> Self {
        Self {
            ast_type: AstType::UnaryExpression {
                op,
                expr
            },
            pine_type: PineType::Unknown,
            span
        }
    }
    
    pub fn new_identifier_expression(identifier: Box<AstNode>, span: Span) -> Self {
        Self {
            ast_type: AstType::IdentifierExpression(identifier),
            pine_type: PineType::Unknown,
            span
        }
    }

    pub fn new_integer_expression(value: i64, span: Span) -> Self {
        Self {
            ast_type: AstType::IntegerExpression(value),
            pine_type: PineType::Unknown,
            span
        }
    }

    pub fn new_float_expression(value: f64, span: Span) -> Self {
        Self {
            ast_type: AstType::FloatExpression(value),
            pine_type: PineType::Unknown,
            span
        }
    }

    pub fn new_string_expression(value: String, span: Span) -> Self {
        Self {
            ast_type: AstType::StringExpression(value),
            pine_type: PineType::Unknown,
            span
        }
    }
    
    pub fn new_identifier(value: String, span: Span) -> Self {
        Self {
            ast_type: AstType::Identifier(value),
            pine_type: PineType::Unknown,
            span,
        }
    }

    pub fn dummy() -> Self {
        Self {
            ast_type: AstType::Dummy,
            pine_type: PineType::Void,
            span: Span::new(Point::new(0, 0), Point::new(0, 0)),
        }
    }
}

pub struct Program {
    pub functions: Vec<AstNode>,
    //pub main: AstNode,
}

impl Program {
    pub fn new(functions: Vec<AstNode>) -> Self {
        Self {
            functions
        }
    }
}
