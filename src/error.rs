use std::fmt;


pub fn new_error(name: &'static str) {}


#[derive(Clone, Debug)]
pub enum LLFeError {
    NOERROR,
    ERROR(ErrorData),
    LEXER(ErrorData),
    PARSER(ErrorData),
    TRANSPILER(ErrorData),
    INTERPRETER(ErrorData),
}


#[derive(Debug)]
pub struct ErrorData {
    pub name: String,
    pub description: String,
    pub caused_by: Option<LLFeError>,
}

impl Default for ErrorData {
    fn default() -> Self {
        Self {
            name: "Error".to_string(),
            description: String::new(),
            caused_by: None
        }
    }
}