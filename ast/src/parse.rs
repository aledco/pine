use crate::ast::*;
use crate::token::*;
use crate::symbol::*;

pub fn parse(tokens: Vec<Token>) -> Program {
    let mut parser = Parser::new(tokens);
    parser.parse()
}

/// Represents the Pine parser
struct Parser {
    /// The vector of tokens representing the Pine program
    tokens: Vec<Token>,
    /// The index into the tokens
    index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, index: 0 }
    }

    pub fn parse(&mut self) -> Program {
        let global_scope = Box::new(Scope::new_global());
        let mut functions: Vec<AstNode> = vec![];
        while !self.eof() {
            if self.matches_function() {
                let function = self.parse_function(global_scope);
                functions.push(function);
            } else {
                panic!("Parse Error: at {}", self.span())
            }
        }

        Program::new(functions)
    }

    fn parse_function(&mut self, scope: &Scope) -> AstNode {
        let param_scope = Scope::new(Some(scope));
        
        let fun = self.match_token(TokenType::Keyword(Keyword::Fun));
        let mut identifier = self.parse_identifier(false);
        let params = self.parse_params();

        // compute the type of the function
        let return_type = if self.matches(TokenType::Punctuation(Punctuation::Arrow)) {
            self.match_token(TokenType::Punctuation(Punctuation::Arrow));
            self.parse_type()
        } else {
            PineType::Void
        };
        let param_types = params
            .iter()
            .map(|p| p.pine_type.clone())
            .collect::<Vec<PineType>>();
        let function_type = PineType::Function {
            params: param_types,
            ret: Box::new(return_type),
        };

        identifier.pine_type = function_type.clone();

        let body = self.parse_block();
        let span = fun.span + body.span;
        AstNode::new_function(
            Box::new(identifier),
            params,
            Box::new(body),
            function_type,
            scope,
            span,
        )
    }

    fn parse_params(&mut self) -> Vec<AstNode> {
        self.match_token(TokenType::Punctuation(Punctuation::OpenParen));
        let mut params = Vec::<AstNode>::new();
        while self.matches_identifier() {
            let param = self.parse_param();
            params.push(param);
            if !self.matches(TokenType::Punctuation(Punctuation::CloseParen)) {
                break;
            }
        }

        self.match_token(TokenType::Punctuation(Punctuation::CloseParen));
        params
    }

    fn parse_param(&mut self) -> AstNode {
        let mut identifier = self.parse_identifier(false);
        self.match_token(TokenType::Punctuation(Punctuation::Colon));
        identifier.pine_type = self.parse_type();
        identifier
    }

    fn parse_identifier(&mut self, lookup: bool) -> AstNode {
        // TODO lookup in symbol table
        let token = self.match_identifier();
        match token.token_type {
            TokenType::Identifier(identifier) => {
                AstNode::new_identifier(identifier.clone(), token.span.clone())
            },
            _ => panic!("Parse Error: at {}", self.span())
        }
    }

    fn parse_block(&mut self) -> AstNode {
        let begin = self.match_any(&vec![
            TokenType::Keyword(Keyword::Begin),
            TokenType::Keyword(Keyword::Then),
            TokenType::Keyword(Keyword::Do),
        ]);
        let mut statements: Vec<AstNode> = vec![];
        while !self.matches(TokenType::Keyword(Keyword::End)) {
            let statement = self.parse_statement();
            statements.push(statement);
        }

        let end = self.match_token(TokenType::Keyword(Keyword::End));
        let span = begin.span + end.span;
        AstNode::new_block(statements, span)
    }

    fn parse_statement(&mut self) -> AstNode {
        if self.matches(TokenType::Keyword(Keyword::Let)) {
            self.parse_let()
        } else if self.matches(TokenType::Keyword(Keyword::If)) {
            self.parse_if()
        } else if self.matches(TokenType::Keyword(Keyword::While)) {
            self.parse_while()
        } else if self.matches(TokenType::Keyword(Keyword::For)) {
            self.parse_for()
        } else if self.matches(TokenType::Keyword(Keyword::Return)) {
            self.parse_return()
        } else if self.matches(TokenType::Keyword(Keyword::Begin)) {
            self.parse_block()
        } else if self.matches_expression() {
            self.parse_expression()
        } else {
            panic!("Parse Error: at {}", self.span())
        }
    }

    fn parse_let(&mut self) -> AstNode {
        let let_token = self.match_token(TokenType::Keyword(Keyword::Let));
        let identifier = self.parse_identifier(false);
        self.match_token(TokenType::Punctuation(Punctuation::EqualSign));
        let expression = self.parse_expression();
        let span = let_token.span + expression.span;
        AstNode::new_let(Box::new(identifier), Box::new(expression), span)
    }

    fn parse_if(&mut self) -> AstNode {
        AstNode::dummy()
    }

    fn parse_while(&mut self) -> AstNode {
        AstNode::dummy()
    }

    fn parse_for(&mut self) -> AstNode {
        AstNode::dummy()
    }

    fn parse_return(&mut self) -> AstNode {
        AstNode::dummy()
    }
    
    fn parse_expression(&mut self) -> AstNode {
        self.parse_expression_by_precedence(1)
    }

    fn parse_expression_by_precedence(&mut self, precedence: i32) -> AstNode {
        let ops: Vec<TokenType> = Operator::get_binary_ops_by_precedence(precedence)
            .into_iter()
            .map(|op| TokenType::Operator(op))
            .collect();

        if ops.len() == 0 {
            self.parse_expression_term()
        } else {
            let mut expr = self.parse_expression_by_precedence(precedence + 1);
            while self.matches_any(&ops) {
                let op = if let TokenType::Operator(op) = self.match_any(&ops).token_type {
                    op
                } else {
                    panic!("Parse Error: at {}", self.span())
                };

                let rhs = self.parse_expression_by_precedence(precedence + 1);
                let span = expr.span + rhs.span;
                expr = AstNode::new_binary_expression(Box::new(expr), op, Box::new(rhs), span);
            }

            expr
        }
    }

    fn parse_expression_term(&mut self) -> AstNode {
        let ops = Operator::get_all_unary_ops()
            .into_iter()
            .map(|op| TokenType::Operator(op))
            .collect();

        if self.matches_identifier() {
            self.parse_identifier_expression()
        } else if self.matches_integer() {
            self.parse_integer()
        } else if self.matches_float() {
            self.parse_float()
        } else if self.matches_string() {
            self.parse_string()
        } else if self.matches_any(&ops) {
            let op_token = self.match_any(&ops);
            let op = if let TokenType::Operator(op) = op_token.token_type {
                op
            } else {
                panic!("Parse Error: at {}", self.span())
            };

            let expr = self.parse_expression();
            let span = op_token.span + expr.span;
            AstNode::new_unary_expression(op, Box::new(expr), span)
        } else if self.matches(TokenType::Punctuation(Punctuation::OpenParen)) {
            self.match_token(TokenType::Punctuation(Punctuation::OpenParen));
            let expr = self.parse_expression();
            self.match_token(TokenType::Punctuation(Punctuation::CloseParen));
            expr
        } else {
            // TODO function calls and array access
            panic!("Parse Error: at {}", self.span())
        }
    }

    fn parse_identifier_expression(&mut self) -> AstNode {
        let identifier = self.parse_identifier(true);
        let span = identifier.span.clone();
        AstNode::new_identifier_expression(Box::new(identifier), span)
    }
    
    fn parse_integer(&mut self) -> AstNode {
        let token = self.match_integer();
        match token.token_type {
            TokenType::Integer(value) => {
                AstNode::new_integer_expression(value, token.span.clone())
            },
            _ => panic!("Parse Error: at {}", self.span())
        }
    }

    fn parse_float(&mut self) -> AstNode {
        let token = self.match_float();
        match token.token_type {
            TokenType::Float(value) => {
                AstNode::new_float_expression(value, token.span.clone())
            },
            _ => panic!("Parse Error: at {}", self.span())
        }
    }

    fn parse_string(&mut self) -> AstNode {
        let token = self.match_string();
        match token.token_type {
            TokenType::String(value) => {
                AstNode::new_string_expression(value, token.span.clone())
            },
            _ => panic!("Parse Error: at {}", self.span())
        }
    }

    fn parse_type(&mut self) -> PineType {
        let span = self.span();
        match self.token_type() {
            TokenType::Keyword(keyword) => {
                self.index += 1;
                match keyword {
                    Keyword::Int => PineType::Integer,
                    Keyword::Float => PineType::Float,
                    Keyword::String => PineType::String,
                    _ => panic!("Parse Error: at {}", span),
                }
            }
            TokenType::Punctuation(p) => {
                match p {
                    Punctuation::OpenBracket => {
                        self.index += 1;
                        let elem_type = self.parse_type();
                        self.match_token(TokenType::Punctuation(Punctuation::OpenBracket));
                        PineType::List(Box::new(elem_type))
                    }
                    Punctuation::OpenParen => {
                        // TODO parse function type
                        panic!("Parse Error: at {}", span)
                    }
                    _ => panic!("Parse Error: at {}", span),
                }
            }
            TokenType::Identifier(_) => {
                self.index += 1;
                PineType::Void // TODO handle user defined types
            }
            _ => panic!("Parse Error: at {}", span),
        }
    }

    fn match_token(&mut self, token_type: TokenType) -> Token {
        let token = self.token(); // TODO consider using strum enum discriminants
        if token.token_type == token_type {
            self.index += 1;
            return token;
        }

        panic!("Parse Error: at {}", token.span)
    }

    fn match_any(&mut self, token_types: &Vec<TokenType>) -> Token {
        for token_type in token_types {
            if self.matches(token_type.clone()) {
                return self.match_token(token_type.clone());
            }
        }

        panic!("Parse Error: at {}", self.span())
    }

    fn match_identifier(&mut self) -> Token {
        let token = self.token();
        if let TokenType::Identifier(_) = token.token_type {
            self.index += 1;
            return token;
        }

        panic!("Parse Error: at {}", token.span)
    }

    fn match_integer(&mut self) -> Token {
        let token = self.token();
        if let TokenType::Integer(_) = token.token_type {
            self.index += 1;
            return token;
        }

        panic!("Parse Error: at {}", token.span)
    }

    fn match_float(&mut self) -> Token {
        let token = self.token();
        if let TokenType::Float(_) = token.token_type {
            self.index += 1;
            return token;
        }

        panic!("Parse Error: at {}", token.span)
    }

    fn match_string(&mut self) -> Token {
        let token = self.token();
        if let TokenType::String(_) = token.token_type {
            self.index += 1;
            return token;
        }

        panic!("Parse Error: at {}", token.span)
    }

    fn matches(&self, token_type: TokenType) -> bool {
        self.token_type() == token_type
    }

    fn matches_any(&self, token_types: &Vec<TokenType>) -> bool {
        token_types.iter().any(|t| self.matches(t.clone()))
    }

    fn matches_function(&self) -> bool {
        self.matches(TokenType::Keyword(Keyword::Fun))
    }

    fn matches_statement(&self) -> bool {
        if self.matches_any(&vec![
            TokenType::Keyword(Keyword::Begin),
            TokenType::Keyword(Keyword::Let),
            TokenType::Keyword(Keyword::If),
            TokenType::Keyword(Keyword::For),
            TokenType::Keyword(Keyword::While),
            TokenType::Keyword(Keyword::Return),
        ]) {
            true
        } else if self.matches_expression() {
            true
        } else {
            false
        }
    }

    fn matches_expression(&self) -> bool {
        if self.matches_identifier()
            || self.matches_integer()
            || self.matches_float()
            || self.matches_string()
        {
            true
        } else if self.matches_any(
            &Operator::get_all_unary_ops()
                .into_iter()
                .map(|o| TokenType::Operator(o))
                .collect(),
        ) {
            true
        } else if self.matches_any(&vec![
            TokenType::Punctuation(Punctuation::OpenParen),
            TokenType::Punctuation(Punctuation::OpenBracket),
        ]) {
            true
        } else {
            false
        }
    }

    fn matches_identifier(&self) -> bool {
        if let TokenType::Identifier(_) = self.token_type() {
            true
        } else {
            false
        }
    }

    fn matches_integer(&self) -> bool {
        if let TokenType::Integer(_) = self.token_type() {
            true
        } else {
            false
        }
    }

    fn matches_float(&self) -> bool {
        if let TokenType::Float(_) = self.token_type() {
            true
        } else {
            false
        }
    }

    fn matches_string(&self) -> bool {
        if let TokenType::String(_) = self.token_type() {
            true
        } else {
            false
        }
    }

    fn token(&self) -> Token {
        self.tokens[self.index].clone()
    }

    fn token_type(&self) -> TokenType {
        self.tokens[self.index].token_type.clone()
    }

    fn span(&self) -> Span {
        self.tokens[self.index].span.clone()
    }

    fn eof(&self) -> bool {
        self.index >= self.tokens.len()
    }
}
