pub mod compiler;
pub mod lexer;
pub mod parser;

pub mod prelude {
    use super::*;

    pub use compiler::Compiler;
    pub use lexer::Lexer;
    pub use parser::Parser;
}
