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

        let mut section_headers: Vec<String> = vec![];
        let mut section_contents: Vec<String> = vec![];

        self.find_section_headers(&mut section_headers)?;
        self.find_section_contents(&section_headers, &mut section_contents)?;

        Ok(tokens)
    }
}



impl Lexer {
    pub fn possible_header_lines(&self) -> Vec<bool> {
        self.0.clone()
            .split("\n")
            .map(str::trim_end)
            .map(|s| {
                let trimmed = s.trim_start();

                (trimmed == s) && (trimmed != "")
            })
            .collect::<Vec<bool>>()
    }

    pub fn nth_line(&self, i: usize) -> Option<String> {
        self.0.clone()
            .split("\n")
            .map(|s| s.trim_end().to_string())
            .nth(i)
    }
}
