pub mod error;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod token;
pub mod transpiler;

pub mod ast;

pub mod prelude {
    pub use super::error::LLFeError;
    pub use super::interpreter::Interpreter;
    pub use super::lexer::Lexer;
    pub use super::parser::Parser;
    pub use super::transpiler::Transpiler;

    pub use super::lex_parse_transpile;
}


pub fn lex_parse_transpile(source: String) -> Result<String, error::LLFeError> {
    let lexer = lexer::Lexer(source);

    let tokens = lexer.lex();

    let parser = parser::Parser(tokens);

    let ast = parser.parse();

    let transpiler = transpiler::Transpiler(ast);

    let arm_source = transpiler.transpile();

    Ok(arm_source)
}


pub fn lex_parse_interpret(source: String) -> Result<(), error::LLFeError> {
    // Create lexer from LLFe source
    let lexer = lexer::Lexer(source);

    // Perform lexical analysis on source
    let tokens = lexer.lex()?;

    // Create parser from tokens
    let parser = parser::Parser(tokens);

    // Perform parsing on tokens
    let ast = parser.parse()?;

    // Create interpreter from AST
    let interpreter = interpreter::Interpreter(ast);

    // Perform transpilation on AST
    interpreter.interpret().unwrap();

    Ok(())
}
