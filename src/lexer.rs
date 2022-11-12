use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

use crate::error::ErrorData;
use crate::error::LLFeError;
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
        self.find_section_contents(&section_headers, &mut section_contents);

        Ok(tokens)
    }

    pub fn find_section_headers(&self, names: &mut Vec<String>) {
        let mut name_captures = SECTION_PATTERN.find_iter(self.0.as_str());

        while let Some(m) = name_captures.next() {
            names.push(m.as_str().to_string());
        }
    }

    pub fn find_section_contents(&self, names: &Vec<String>, contents: &mut Vec<String>) -> Vec<bool> {
        let names = names.iter();

        let mut source_lines = self.0
            .split("\n")
            .map(|s| s.trim_end().to_string())
            .collect::<Vec<String>>();

        let possible_header_lines = source_lines
            .iter()
            .map(|s| {
                // println!("'{}', '{}'", s, s.trim_start());
                let trimmed = s.trim_start();

                (trimmed == s.as_str()) && (trimmed != "")
            })
            .collect::<Vec<bool>>();

        possible_header_lines
    }
}
