#[derive(Clone, Debug)]
pub enum LLFeError {
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

impl ErrorData {
    fn format_name(e: Self, name: &'static str) -> Self {
        Self {
            name: format!("{}{}", name, e.name),
            description: e.description,
            caused_by: e.caused_by,
        }
    }
}

pub fn handle_error(e: LLFeError) -> ! {
    let msg = match e {
        LLFeError::ERROR(e) => { ErrorData::format_name(e, "Error: ") }
        LLFeError::LEXER(e) => { ErrorData::format_name(e, "Lexer: ") }
        LLFeError::PARSER(e) => { ErrorData::format_name(e, "Parser: ")}
        LLFeError::TRANSPILER(e) => { ErrorData::format_name(e, "Transpiler: ")}
        LLFeError::INTERPRETER(e) => { ErrorData::format_name(e, "Interpreter: ")}
    };

    panic!("{msg:?}");
}