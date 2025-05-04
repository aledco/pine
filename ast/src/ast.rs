use crate::symbol::*;
use crate::token::*;
use std::fmt;

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<AstNode>,
    //pub main: AstNode,
}

impl Program {
    pub fn new(functions: Vec<AstNode>) -> Self {
        Self { functions }
    }
}

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

#[derive(Debug)]
pub enum AstType {
    Function {
        identifier: Box<AstNode>,
        params: Vec<AstNode>,
        body: Box<AstNode>,
    },
    Block(Vec<AstNode>),
    IfStatement{
        condition: Box<AstNode>,
        then_body: Box<AstNode>,
        else_body: Option<Box<AstNode>>,
    },
    WhileStatement{
        condition: Box<AstNode>,
        body: Box<AstNode>,
    },
    ReturnStatement(Option<Box<AstNode>>),
    LetStatement {
        identifier: Box<AstNode>,
        expression: Box<AstNode>,
    },
    SetStatement {
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
    Identifier(SymbolRef),
    Dummy,
}

pub struct AstNode {
    pub ast_type: AstType,
    pub pine_type: PineType,
    pub scope: ScopeRef,
    pub span: Span,
}

impl AstNode {
    pub fn new_function(
        identifier: Box<AstNode>,
        params: Vec<AstNode>,
        body: Box<AstNode>,
        pine_type: PineType,
        scope: ScopeRef,
        span: Span,
    ) -> Self {
        Self {
            ast_type: AstType::Function {
                identifier,
                params,
                body,
            },
            pine_type,
            scope,
            span,
        }
    }

    pub fn new_block(statements: Vec<AstNode>, scope: ScopeRef, span: Span) -> Self {
        Self {
            ast_type: AstType::Block(statements),
            pine_type: PineType::Void,
            scope,
            span,
        }
    }

    pub fn new_if_statement(
        condition: Box<AstNode>,
        then_body: Box<AstNode>,
        else_body: Option<Box<AstNode>>,
        scope: ScopeRef,
        span: Span,
    ) -> Self {
        Self {
            ast_type: AstType::IfStatement{
                condition,
                then_body,
                else_body
            },
            pine_type: PineType::Void,
            scope,
            span,
        }
    }

    pub fn new_while_statement(
        condition: Box<AstNode>,
        body: Box<AstNode>,
        scope: ScopeRef,
        span: Span,
    ) -> Self {
        Self {
            ast_type: AstType::WhileStatement{
                condition,
                body
            },
            pine_type: PineType::Void,
            scope,
            span,
        }
    }
    
    pub fn new_return_statement(
        expression: Option<Box<AstNode>>,
        scope: ScopeRef,
        span: Span,
    ) -> Self {
        Self {
            ast_type: AstType::ReturnStatement(expression),
            pine_type: PineType::Void,
            scope,
            span,
        }
    }

    pub fn new_let_statement(
        identifier: Box<AstNode>,
        expression: Box<AstNode>,
        scope: ScopeRef,
        span: Span,
    ) -> Self {
        Self {
            ast_type: AstType::LetStatement {
                identifier,
                expression,
            },
            pine_type: PineType::Void,
            scope,
            span,
        }
    }

    pub fn new_set_statement(
        identifier: Box<AstNode>,
        expression: Box<AstNode>,
        scope: ScopeRef,
        span: Span,
    ) -> Self {
        Self {
            ast_type: AstType::SetStatement {
                identifier,
                expression,
            },
            pine_type: PineType::Void,
            scope,
            span,
        }
    }

    pub fn new_binary_expression(
        lhs: Box<AstNode>,
        op: Operator,
        rhs: Box<AstNode>,
        scope: ScopeRef,
        span: Span,
    ) -> Self {
        Self {
            ast_type: AstType::BinaryExpression { lhs, op, rhs },
            pine_type: PineType::Unknown,
            scope,
            span,
        }
    }

    pub fn new_unary_expression(
        op: Operator,
        expr: Box<AstNode>,
        scope: ScopeRef,
        span: Span,
    ) -> Self {
        Self {
            ast_type: AstType::UnaryExpression { op, expr },
            pine_type: PineType::Unknown,
            scope,
            span,
        }
    }

    pub fn new_identifier_expression(
        identifier: Box<AstNode>,
        scope: ScopeRef,
        span: Span,
    ) -> Self {
        Self {
            ast_type: AstType::IdentifierExpression(identifier),
            pine_type: PineType::Unknown,
            scope,
            span,
        }
    }

    pub fn new_integer_expression(value: i64, scope: ScopeRef, span: Span) -> Self {
        Self {
            ast_type: AstType::IntegerExpression(value),
            pine_type: PineType::Unknown,
            scope,
            span,
        }
    }

    pub fn new_float_expression(value: f64, scope: ScopeRef, span: Span) -> Self {
        Self {
            ast_type: AstType::FloatExpression(value),
            pine_type: PineType::Unknown,
            scope,
            span,
        }
    }

    pub fn new_string_expression(value: String, scope: ScopeRef, span: Span) -> Self {
        Self {
            ast_type: AstType::StringExpression(value),
            pine_type: PineType::Unknown,
            scope,
            span,
        }
    }

    pub fn new_identifier(value: SymbolRef, scope: ScopeRef, span: Span) -> Self {
        Self {
            ast_type: AstType::Identifier(value),
            pine_type: PineType::Unknown,
            scope,
            span,
        }
    }

    pub fn depth(&self) -> usize {
        match self.scope.borrow().depth {
            ScopeDepth::Global => 0,
            ScopeDepth::Local(depth) => depth,
        }
    }

    pub fn dummy() -> Self {
        Self {
            ast_type: AstType::Dummy,
            pine_type: PineType::Void,
            scope: Scope::new_global(),
            span: Span::new(Point::new(0, 0), Point::new(0, 0)),
        }
    }
}

impl fmt::Debug for AstNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        fmt::Debug::fmt(&self.ast_type, f)
    }
}
