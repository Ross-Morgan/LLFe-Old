use std::fmt::Debug;

#[derive(Clone)]
pub enum LLFeError {
    ERROR(String),
    LEXER(String),
    PARSER(String),
    TRANSPILER(String),
}


impl Default for LLFeError {
    fn default() -> Self {
        Self::ERROR("".to_string())
    }
}

impl Debug for LLFeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}