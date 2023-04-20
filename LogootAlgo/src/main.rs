pub mod uuid;
pub mod document;
use uuid::UUID;
use std::fs::File;
use std::io::prelude::*;

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

fn main() {    
    // 100 files in ../data with versions of a Wikipedia article

    // Read in the first version ../data/0.txt as a list of strings
    let path = "LogootLab/data/0.txt";
    let mut file = File::open(path).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("something went wrong reading the file");

    // Split by new line and add to array of strings
    let mut lines: Vec<String> = Vec::new();
    for line in contents.lines() {
        lines.push(line.to_string());
    }

    // Create a document with the lines
    let mut document = Document { lines: Vec::new() };
    



    // Store the diffs from 0..1..2..3..4..5..6..7..8..9


}