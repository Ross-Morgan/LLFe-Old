use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

use crate::error::{ErrorData, LLFeError};
use crate::Tokens;

lazy_static! {
    static ref SECTION_PATTERN: Regex = Regex::new(r"(?gm)
    ^\w+:$
    ").unwrap();
}


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
        //TODO Implement lexer
        let mut tokens = vec![];

        let mut section_headers: Vec<String> = vec![];
        let mut section_contents: Vec<String> = vec![];

        self.find_section_headers(&mut section_headers);
        self.find_section_contents(&section_headers, &mut section_contents)?;

        Ok(tokens)
    }
}

impl Lexer {
    pub fn find_section_headers(&self, names: &mut Vec<String>) -> Result<(), LLFeError> {
        // Find all matches in source
        let mut name_captures = SECTION_PATTERN.find_iter(self.0.as_str());

        // Loop through matches
        while let Some(m) = name_captures.next() {
            // Push header name to name vec
            let header_name = m.as_str().to_string();

            if header_name.get(0..(header_name.len() - 1)) != Some(":") {
                return Err(LLFeError::LEXER(ErrorData {
                    name: "".to_string(),
                    description: "".to_string(),
                    caused_by: Box::new(None)
                }));
            }

            names.push(header_name);
        }

        Ok(())
    }

    pub fn find_section_contents(&self, names: &Vec<String>, contents: &mut Vec<String>) -> Result<(), LLFeError> {
        let names = names.iter();

        // Split source into lines
        let mut source_lines = self.0
            .split("\n")
            .map(|s| s.trim_end().to_string())
            .collect::<Vec<String>>();

        // Check if lines can be headers
        let possible_header_lines = source_lines
            .iter()
            .map(|s| {
                let trimmed = s.trim_start();

                (trimmed == s.as_str()) && (trimmed != "")
            })
            .collect::<Vec<bool>>();

        // Extract header lines
        let headers = {
            let mut v = vec![];

            for (i, line_is_header) in possible_header_lines.into_iter().enumerate() {
                if line_is_header {
                    v.push(source_lines.get(i).clone().unwrap());
                }
            }

            v
        };

        // Check for invalid headers
        super::checks::are_headers_valid(&headers)?;

        Ok(())
    }
}
