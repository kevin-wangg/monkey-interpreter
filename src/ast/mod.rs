mod tests;

use std::any::Any;

use crate::token::Token;

/// Represents a node in the AST. Each node implements the `token_literal` function, which
/// is mainly used for debugging purposes. It returns the literal of the token associated
/// with this node.
pub trait Node {
    fn token_literal(&self) -> String;
    fn as_any(&self) -> &dyn Any;
    fn string(&self) -> String;
}

pub trait Statement: Node {}

pub trait Expression: Node {}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Program {
    pub fn new(statements: Vec<Box<dyn Statement>>) -> Self {
        Program { statements }
    }
}

// ========== Identifier Start ==========

pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Identifier {
    pub fn new(token: Token, value: &str) -> Self {
        Identifier {
            token,
            value: value.to_string(),
        }
    }
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn string(&self) -> String {
        self.value.clone()
    }
}

impl Expression for Identifier {}

// ========== Identifier End ==========

// ========== Let statement Start ==========

pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    // TODO: Uncomment this when parsing Expressions is supported
    // pub value: Box<dyn Expression>,
}

impl LetStatement {
    // TODO: Add a value parameter when parsing Expressions is supported
    pub fn new(token: Token, name: Identifier) -> Self {
        LetStatement { token, name }
    }
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn string(&self) -> String {
        format!("let {} = <placeholder>;", self.name.string())
    }
}

impl Statement for LetStatement {}

// ========== Let statement End ==========

// ========== Return statement Start ==========

// TODO: Revisit this once expression parsing is supported
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Box<dyn Expression>,
}

// ========== Return statement End ==========

pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Box<dyn Expression>,
}

impl ExpressionStatement {}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn string(&self) -> String {
        format!("{};", self.expression.string())
    }
}

impl Statement for ExpressionStatement {}
