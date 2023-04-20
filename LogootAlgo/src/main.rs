pub mod uuid;
pub mod document;
use document::Document;
use uuid::UUID;
use std::fs::File;
use std::io::prelude::*;
use similar::{ChangeTag, TextDiff};

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub struct Line {
    identifier: UUID,
    content: String,
}

// create structure for storing diffs, basically a diff type and potentially a line
#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub struct Diff {
    diff_type: ChangeTag,
    line: String,
    index: usize,
}

fn main() {
    // 100 files in ../data with versions of a Wikipedia article

    // Read in the first version ../data/0.txt as a list of strings
    let path = "../DataProcessing/text/0.txt";
    let mut file = File::open(path).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("something went wrong reading the file");

    // Split by new line and add to array of strings
    let mut lines: Vec<String> = Vec::new();
    for line in contents.lines() {
        lines.push(line.to_string());
    }

    // Create a document with the lines
    let mut document = Document::new(lines);

    // Store the diffs from 0..1..2..3..4..5..6..7..8..9
    let mut diffs: Vec<Vec<Diff>> = Vec::new();

    for i in 1..100 {
        // Read in the next version ../data/i.txt as a list of strings
        let path = format!("../DataProcessing/text/{}.txt", i);
        let mut new_content: String = String::new();
        let mut file = File::open(path).expect("file not found");

        file.read_to_string(&mut new_content).expect("something went wrong reading the file");

        let diff = TextDiff::from_lines(&contents, &new_content);

        let mut diff_vec: Vec<Diff>= Vec::new();
        for change in diff.iter_all_changes() {
            match change.tag() {
                ChangeTag::Delete => {
                    let diff = Diff {
                        diff_type: ChangeTag::Delete,
                        line: change.clone().to_string(),
                        index: change.clone().old_index().unwrap(),
                    };
                    diff_vec.push(diff);
                },
                ChangeTag::Insert => {
                    let diff = Diff {
                        diff_type: ChangeTag::Insert,
                        line: change.clone().to_string(),
                        index: change.clone().new_index().unwrap(),
                    };
                    diff_vec.push(diff);
                },
                ChangeTag::Equal => {}
            }
        }

        diffs.push(diff_vec);

        contents = new_content;
    }


}