use crate::error::{LLFeError, ErrorData};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref HEADER_PATTERN: Regex = Regex::new(r"(?gm)
    ^[a-z_]+$
    ").unwrap();
}

pub fn are_headers_valid(headers: &Vec<&String>) -> Result<(), LLFeError> {
    for header in headers {
        if !HEADER_PATTERN.is_match(header.as_str()) {
            return Err(LLFeError::LEXER(ErrorData {
                name: "Invalid Header Error".to_string(),
                description: format!("Header `{header}` is invalid"),
                caused_by: Box::new(None),
            }));
        }
    }

     Ok(())
}
