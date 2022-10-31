use crate::ast;

type Tokens = Vec<crate::token::Token>;

pub struct Parser(pub(crate) Tokens);


impl Parser {
    pub fn parse(&self) -> ast::AST {
        //TODO Implement parser
        let mut tree = ast::AST::new();

        tree.push(ast::nodes::Node::Node);

        tree
    }
}