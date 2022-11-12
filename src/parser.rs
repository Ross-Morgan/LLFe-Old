use crate::{ast, prelude::LLFeError};

type Tokens = Vec<crate::token::Token>;

pub struct Parser(pub(crate) Tokens);


impl Parser {
    pub fn parse(&self) -> Result<ast::AST, LLFeError> {
        //TODO Implement parser
        let mut tree = ast::AST::new();

        tree.push(ast::nodes::Node::Node);

        Ok(tree)
    }
}