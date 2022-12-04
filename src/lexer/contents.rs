use std::collections::HashMap;

use regex::Regex;
use lazy_static::lazy_static;

use crate::error::LLFeError;


lazy_static! {
    static ref NAME_PATTERN: Regex = Regex::new("^[a-z_]+:").unwrap();
}

impl super::Lexer {
    pub fn find_section_contents(&self) -> Result<Vec<String>, LLFeError> {
        let mut contents = vec![];

        Ok(contents)
    }
}