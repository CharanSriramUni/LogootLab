pub mod uuid;
use uuid::UUID;

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub struct Identifier {
    position: usize,
    site_id: UUID
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub struct Line {
    identifier: Identifier,
    content: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Document {
    lines: Vec<Line>,
}

fn main() {    
    
}
