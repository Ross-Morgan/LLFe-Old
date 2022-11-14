use std::collections::HashMap;
use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

use crate::error::{ErrorData, LLFeError};
use crate::Tokens;

lazy_static! {
    static ref SECTION_PATTERN: Regex = Regex::new(r"^\w+:$").unwrap();
}


pub struct Lexer(pub String);


impl Lexer {
    pub fn lex_file(filename: &str) -> Tokens {
        Self::try_lex_file(filename).expect("Failed to open file")
    }

    pub fn try_lex_file(filename: &str) -> Result<Tokens, LLFeError> {
        let source_contents = fs::read_to_string(filename);

        let source_contents = match source_contents {
            Ok(val) => { Ok(val) }
            Err(e) => {
                Err(LLFeError::ERROR(ErrorData {
                    name: "File Error".to_string(),
                    description: format!("{e}"),
                    caused_by: Box::new(None),
                }))
            }
        }?;

        let lexer = Self(source_contents);
        let tokens = lexer.lex()?;

        Ok(tokens)
    }
}


impl Lexer {
    pub fn lex(&self) -> Result<Tokens, LLFeError> {
        let mut tokens = vec![];

        let mut section_headers: Vec<String> = vec![];
        let mut section_contents: Vec<String> = vec![];

        self.find_section_headers(&mut section_headers)?;
        self.find_section_contents(&section_headers, &mut section_contents)?;

        Ok(tokens)
    }
}

impl Lexer {
    pub fn find_section_headers(&self, names: &mut Vec<String>) -> Result<(), LLFeError> {
        // Find all matches in source
        let mut name_captures = SECTION_PATTERN.find_iter(self.0.as_str());

        // Loop through matches
        while let Some(m) = name_captures.next() {
            // Push header name to name vec
            let header_name = m.as_str().to_string();

            if header_name.get(0..(header_name.len() - 1)) != Some(":") {
                return Err(LLFeError::LEXER(ErrorData {
                    name: "".to_string(),
                    description: "".to_string(),
                    caused_by: Box::new(None)
                }));
            }

            names.push(header_name);
        }

        Ok(())
    }

    pub fn find_section_contents(&self, names: &Vec<String>, contents: &mut Vec<String>) -> Result<(), LLFeError> {
        fill_vec(contents, names.len(), String::new);

        let possible_header_lines = self.possible_header_lines();

        // Extract header lines
        let headers = {
            let mut v = vec![];

            for (i, line_is_header) in possible_header_lines.into_iter().enumerate() {
                if line_is_header {
                    v.push(self.nth_line(i).unwrap());
                }
            }

            v
        };

        super::checks::are_headers_valid(&headers)?;
        super::checks::does_entry_header_exist(&headers)?;

        let mut sections = HashMap::<String, String>::new();

        // Find whitespace characters at start of line
        // Find regularity in whitespace (0 or n)
        // Each chunk of lines starting with whitespace is a section

        let whitespace_at_start_of_line = self.0
            .replace("\r", "")
            .split("\n")
            .map(|s| {
                // Empty lines count as whitespace
                if s == "" { return true; }

                // Check if first character is whitespace
                s.chars().nth(0).unwrap().is_ascii_whitespace()
            })
            .collect::<Vec<bool>>();

        println!("{whitespace_at_start_of_line:?}");

        let mut section_buffer = String::new();
        let mut collecting_section_name = String::new();

        for (i, has_whitespace) in whitespace_at_start_of_line.into_iter().enumerate() {
            // Reached new section
            match has_whitespace {
                true => {}
                false => {
                    // If anything was collected and a section name exists, add section contents with name
                    if !section_buffer.is_empty() && collecting_section_name != "" {
                        sections.insert(collecting_section_name.clone(), section_buffer.clone());
                    }

                    collecting_section_name = self.nth_line(i).unwrap();
                }
            }
        }

        Ok(())
    }
}

impl Lexer {
    pub fn possible_header_lines(&self) -> Vec<bool> {
        self.0.clone()
            .split("\n")
            .map(str::trim_end)
            .map(|s| {
                let trimmed = s.trim_start();

                (trimmed == s) && (trimmed != "")
            })
            .collect::<Vec<bool>>()
    }

    pub fn nth_line(&self, i: usize) -> Option<String> {
        self.0.clone()
            .split("\n")
            .map(|s| s.trim_end().to_string())
            .nth(i)
    }
}


fn fill_vec<T>(v: &mut Vec<T>, n: usize, p: fn() -> T) {
    v.clear();

    unsafe { v.set_len(n); }

    for _ in 0..n {
        v.push(p());
    }
}


fn longest_identical_subsequence_of<T>(vec: &Vec<T>, val: T) -> (usize, usize) {
    let mut longest: usize; 

    (0, 0)
}