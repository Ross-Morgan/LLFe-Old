pub use crate::ast::{
    AST,
    nodes::Node,
};


#[derive(Clone, Debug)]
pub struct Interpreter(pub AST);

impl Interpreter {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn add_ast_node(&mut self, node: Node) {
        self.0.push(node);
    }

    pub fn interpret(&self) {}
}