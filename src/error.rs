use std::fmt;


#[derive(Clone, Debug)]
pub enum LLFeError {
    NO_ERROR,
    ERROR(ErrorData<"Error">),
    LEXER(ErrorData<"Lexer">),
    PARSER(ErrorData<"Parser">),
    TRANSPILER(ErrorData<"Transpiler">),
    INTERPRETER(ErrorData<"Interpreter">),
}


pub struct ErrorData<const S: &'static str> {
    pub description: String
    pub caused_by: Option<LLFeError>
}


impl<const S: &'static str> fmt::Debug for ErrorData<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ErrorData")
         .field("type", S.clone())
         .field("desc", &self.description)
         .field("caused by", &self.caused_by.unwrap_or(LLFeError::NO_ERROR))
         .finish()
    }
}

impl Default for ErrorData {
    fn default() -> Self {
        Self {
            description: String::new(),
            caused_by: None
        }
    }
}