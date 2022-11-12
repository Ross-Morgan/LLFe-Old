#[allow(non_camel_case_types)]
pub enum Token {
    KW_MODE(String),
    KW_LOAD(String, Box<Self>),
    KW_CALL,

    SECTION_LABEL(String)
}
