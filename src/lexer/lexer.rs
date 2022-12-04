use std::fs;

use crate::error::{ErrorData, LLFeError};
use crate::Tokens;

pub struct Lexer(pub String);

impl Lexer {
    pub fn lex_file(filename: &str) -> Tokens {
        Self::try_lex_file(filename).expect("Failed to open file")
    }

    pub fn try_lex_file(filename: &str) -> Result<Tokens, LLFeError> {
        let source_contents = fs::read_to_string(filename);

        let source_contents = match source_contents {
            Ok(val) => { Ok(val) }
            Err(e) => {
                Err(LLFeError::ERROR(ErrorData {
                    name: "File Error".to_string(),
                    description: format!("{e}"),
                    caused_by: Box::new(None),
                }))
            }
        }?;

        let lexer = Self(source_contents);
        let tokens = lexer.lex()?;

        Ok(tokens)
    }
}


impl Lexer {
    pub fn lex(&self) -> Result<Tokens, LLFeError> {
        let tokens = vec![];

        for c in self.0.chars() {

        }

        Ok(tokens)
    }
}

impl Lexer {
    pub fn nth_line(&self, i: usize) -> Option<String> {
        self.0.clone()
            .split("\n")
            .map(|s| s.trim_end().to_string())
            .nth(i)
    }
}
