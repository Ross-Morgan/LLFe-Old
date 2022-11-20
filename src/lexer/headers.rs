use lazy_static::lazy_static;
use regex::Regex;

use crate::error::{ErrorData, LLFeError};

lazy_static! {
    static ref HEADER_PATTERN: Regex = Regex::new(r"^[a-z][a-z_]*:$").unwrap();
}

impl super::Lexer {
    pub fn find_section_names(&self) -> Result<Vec<String>, LLFeError> {
        let mut names = vec![];

        for line in self.0.split("\n") {
            if !HEADER_PATTERN.is_match(line) {
                return Err(LLFeError::LEXER(ErrorData {
                    name: "Invalid header".to_string(),
                    description: "".to_string(),
                    caused_by: Box::new(None)
                }))
            }

            names.push(line.replace(":", ""));
        }

        Ok(names)
    }
}