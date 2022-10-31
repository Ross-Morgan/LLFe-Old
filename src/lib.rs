pub mod error;
pub mod lexer;
pub mod parser;
pub mod token;
pub mod transpiler;

pub mod ast;

pub mod prelude {
    pub use super::error::LLFeError;
    pub use super::lexer::Lexer;
    pub use super::parser::Parser;
    pub use super::transpiler::Transpiler;

    pub use super::lex_parse_transpile;
}


pub fn lex_parse_transpile(source: String) -> Result<(), error::LLFeError> {
    let lexer = lexer::Lexer(source);

    let tokens = lexer.lex();

    let parser = parser::Parser(tokens);

    let ast = parser.parse();

    let transpiler = transpiler::Transpiler(ast);

    transpiler.transpile();

    Ok(())
}
