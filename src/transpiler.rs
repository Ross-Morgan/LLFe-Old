use crate::ast;

pub struct Transpiler(pub(crate) ast::AST);


impl Transpiler {
    pub fn transpile(&self) -> String {
        //TODO Implement transpiler
        String::new()
    }
}