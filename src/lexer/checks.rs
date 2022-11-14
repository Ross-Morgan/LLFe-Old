use crate::error::{LLFeError, ErrorData};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref HEADER_PATTERN: Regex = Regex::new(r"^[a-z_]+:$").unwrap();
}

pub fn are_headers_valid(headers: &Vec<String>) -> Result<(), LLFeError> {
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

pub fn does_entry_header_exist(headers: &Vec<String>) -> Result<(), LLFeError> {
    let pos = headers.iter().position(|x| x == "entry:");

    if pos.is_none() {
        return Err(LLFeError::LEXER(ErrorData {
            name: "".to_string(),
            description: "".to_string(),
            caused_by: Box::new(None),
        }));
    }

    Ok(())
}
