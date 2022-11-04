use std::fs;
use std::io;

use crate::error::LLFeError;
use crate::token::Token;


type Tokens = Vec<Token>;

pub struct Lexer(pub(crate) String);


impl Lexer {
    pub fn lex_file(filename: &str) -> Tokens {
        Self::try_lex_file(filename).expect("Failed to open file")
    }

    pub fn try_lex_file(filename: &str) -> Result<Tokens, io::Error> {
        let source_contents = fs::read_to_string(filename)?;
        let lexer = Lexer(source_contents);

        Ok(lexer.lex())
    }
}


impl Lexer {
    pub fn lex(&self) -> Result<Tokens, LLFeError> {
        //TODO Implement lexer

        vec![]
    }
}
