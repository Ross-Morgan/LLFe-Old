use crate::ast::AST;

pub struct Interpreter {
    source: String
}

impl Interpreter {
    pub fn new() -> Self {
        Self { source: String::new() }
    }

    pub fn interpret(&self, ast: AST)
}