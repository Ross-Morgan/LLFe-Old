use std::collections::HashMap;

use crate::utils;
use crate::error::LLFeError;


impl super::Lexer {
    pub fn find_section_contents(&self, names: &Vec<String>, contents: &mut Vec<String>) -> Result<(), LLFeError> {
        utils::fill_vec(contents, names.len(), String::new);

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