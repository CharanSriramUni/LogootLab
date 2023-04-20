use crate::{uuid::{UUID}, Line, Identifier};

#[derive(Debug, PartialEq, Eq)]
pub struct Document {
    lines: Vec<Line>,
}

impl Document {
    pub fn new(lines: Vec<String>) -> Document {
        let mut document = Document { lines: Vec::new() };
        
        // Create start line
        let start_identifier = Identifier { position: 0, site_id: UUID::create_start() };
        let start_line = Line { identifier: start_identifier, content: String::from("") }; // Empty content since this is the start line

        // Create end line
        let end_identifier = Identifier { position: 0, site_id: UUID::create_end() };
        let end_line = Line { identifier: end_identifier, content: String::from("") }; // Empty content since this is the start line

        for line in lines {
            
            
        }
        document
    }
}


