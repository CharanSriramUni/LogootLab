use crate::{uuid::{PID}, Line};

impl Ord for Line {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.identifier.cmp(&other.identifier)
    }
}


#[derive(Debug, PartialEq, Eq)]
pub struct Document {
    lines: Vec<Line>,
}

impl Document {
    pub fn new(lines: Vec<String>) -> Document {
        let mut document = Document { lines: Vec::new() };
        
        // Create start line
        let start_identifier = PID::create_start();
        let start_line = Line { identifier: start_identifier, content: String::from("") }; // Empty content since this is the start line

        // Create end line
        let end_identifier = PID::create_end();
        let end_line = Line { identifier: end_identifier, content: String::from("") }; // Empty content since this is the start line


        document.lines.push(start_line);
        let mut last_line = &document.lines[document.lines.len() - 1];


        for line in lines {
            
            let identifier = PID::generate_between(&last_line.identifier, &end_line.identifier);
            let line = Line { identifier, content: line };
            
            document.lines.push(line);   
                    
            last_line = &document.lines[document.lines.len() - 1];
        }

        document.lines.push(end_line);

        document
    }

    // Adds a line to the document. You may need to search the document for the right place to insert the line
    pub fn add(&mut self, uuid: PID, content: String) {
        
    }

    // Convert to a binary search version
    pub fn remove(&mut self, identifier: &PID) {
        let mut index = 0;
        for line in &self.lines {
            if line.identifier == *identifier {
                self.lines.remove(index);
                break;
            }
            index += 1;
        }
    }

    // Retrieve the lines of the document
    pub fn get_lines(&self) -> &Vec<Line> {
        &self.lines
    }
}


