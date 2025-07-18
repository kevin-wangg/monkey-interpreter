use std::any::Any;

use dyn_clone::DynClone;

use crate::ast::{BlockStatement, Identifier, Node};
use crate::evaluator::environment::Environment;

pub trait Object: Any + DynClone {
    fn as_any(&self) -> &dyn Any;
    fn inspect(&self) -> String;
}

dyn_clone::clone_trait_object!(Object);

// ========== Integer Start ==========

#[derive(Clone)]
pub struct Integer {
    pub value: i64,
}

impl Object for Integer {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn inspect(&self) -> String {
        self.value.to_string()
    }
}

impl Integer {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
}

// ========== Integer End ==========

// ========== Boolean Start ==========

#[derive(Clone)]
pub struct Boolean {
    pub value: bool,
}

impl Object for Boolean {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn inspect(&self) -> String {
        self.value.to_string()
    }
}

impl Boolean {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
}

// ========== Boolean End ==========

// ========== Null Start ==========

#[derive(Clone)]
pub struct Null {}

impl Object for Null {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn inspect(&self) -> String {
        "null".to_string()
    }
}

impl Null {
    pub fn new() -> Self {
        Self {}
    }
}

// ========== Null End ==========

// ========== Function Start ==========

#[derive(Clone)]
pub struct Function {
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
    // When env is None, this means the function is not a closure, ie. it does not
    // capture its environment. Recursive functions must not be closures in Monkey Lang.
    // I should probably create a separate struct for this...
    pub env: Option<Environment>,
}

impl Object for Function {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn inspect(&self) -> String {
        let parameter_string = self
            .parameters
            .iter()
            .map(|identifier| identifier.string())
            .collect::<Vec<_>>()
            .join(",");
        format!("fun({}) {}", parameter_string, self.body.string())
    }
}

impl Function {
    pub fn new(parameters: &[Identifier], body: BlockStatement, env: Option<Environment>) -> Self {
        Self {
            parameters: parameters.to_vec(),
            body,
            env,
        }
    }
}

// ========== Function End ==========

// ========== ReturnValue Start ==========

#[derive(Clone)]
pub struct ReturnValue {
    pub value: Box<dyn Object>,
}

impl Object for ReturnValue {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn inspect(&self) -> String {
        self.value.inspect()
    }
}

impl ReturnValue {
    pub fn new(value: Box<dyn Object>) -> Self {
        Self { value }
    }
}

// ========== ReturnValue End ==========
