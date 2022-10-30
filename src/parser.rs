trait Token {}

pub struct Parser<'a> {
    tokens: Vec<&'a mut dyn Token>
}

impl Parser<'_> {
    pub fn new() -> Self {
        Self { tokens: vec![] }
    }
}