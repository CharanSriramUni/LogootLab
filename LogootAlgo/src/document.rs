use crate::{uuid::{UUID}, Line};

#[derive(Debug, PartialEq, Eq)]
pub struct Document {
    lines: Vec<Line>,
}

impl Document {
    pub fn new(lines: Vec<String>) -> Document {
        let mut document = Document { lines: Vec::new() };
        
        // Create start line
        let start_identifier = UUID::create_start();
        let start_line = Line { identifier: start_identifier, content: String::from("") }; // Empty content since this is the start line

        // Create end line
        let end_identifier = UUID::create_end();
        let end_line = Line { identifier: end_identifier, content: String::from("") }; // Empty content since this is the start line

        let mut last_line = &start_line;

        for line in lines {

            // If we've just started, insert between start and end
            if last_line == &start_line {
                let identifier = UUID::generate_between(&start_line.identifier, &end_line.identifier);
                let line = Line { identifier, content: line };
                
                document.lines.push(line);
            } else {
                let identifier = UUID::generate_between(&last_line.identifier, &end_line.identifier);
                let line = Line { identifier, content: line };
                
                document.lines.push(line);   
            }
            
            last_line = &document.lines[document.lines.len() - 1];
        }

        document.lines.push(start_line);
        document.lines.push(end_line);

        document
    }
}


