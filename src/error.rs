#[derive(Clone, Debug)]
pub enum LLFeError {
    NOERROR,
    ERROR(ErrorData),
    LEXER(ErrorData),
    PARSER(ErrorData),
    TRANSPILER(ErrorData),
    INTERPRETER(ErrorData),
}


#[derive(Clone, Debug)]
pub struct ErrorData {
    pub name: String,
    pub description: String,
    pub caused_by: Box<Option<LLFeError>>,
}

impl Default for ErrorData {
    fn default() -> Self {
        Self {
            name: "Error".to_string(),
            description: String::new(),
            caused_by: Box::new(None)
        }
    }
}