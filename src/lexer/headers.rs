use lazy_static::lazy_static;
use regex::Regex;

use crate::error::{ErrorData, LLFeError};

lazy_static! {
    static ref SECTION_PATTERN: Regex = Regex::new(r"^\w+:$").unwrap();
}

impl super::Lexer {
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
}