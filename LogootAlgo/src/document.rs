use uuid::UUID;


#[derive(Debug, PartialEq, Eq)]
pub struct Document {
    lines: Vec<Line>,
}


fn createLogootDocument(lines: Vec<String>) -> Document {
    let mut document = Document { lines: Vec::new() };
    for line in lines {
        let identifier = Identifier { position: 0, site_id: UUID::new() };
        let line = Line { identifier: identifier, content: line };
        document.lines.push(line);
    }
    document
}
