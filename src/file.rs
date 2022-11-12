use std::fs;

use crate::error::{LLFeError, ErrorData};

pub fn load_file(filename: &str, trim_ends: bool) -> Result<String, LLFeError> {
    let source = fs::read_to_string(filename);

    // If `Err` variant, Wrap error value in LLFeError
    match source {
        Ok(mut v) => {
            if trim_ends {
                v = v.trim().to_string();
            }

            Ok(v)
        }
        Err(e) => {
            Err(LLFeError::ERROR(ErrorData {
                name: "File Error".to_string(),
                description: format!("{e}"),
                caused_by: Box::new(None),
            }))
        }
    }
}