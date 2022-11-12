use crate::ast;
use crate::error::LLFeError;

pub struct Transpiler(pub(crate) ast::AST);


impl Transpiler {
    pub fn transpile(&self) -> Result<String, LLFeError> {
        //TODO Implement transpiler
        Ok(String::new())
    }
}