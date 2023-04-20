pub mod uuid;
pub mod document;
use document::Document;
use std::{fs::File};
use std::io::prelude::*;
use similar::{ChangeTag, TextDiff};


// create structure for storing diffs, basically a diff type and potentially a line
#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
pub struct Diff {
    diff_type: ChangeTag,
    line: String,
    index: usize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
pub struct Patch {
    diff_type : ChangeTag,
    diffs: Vec<String>,
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
    // only first 10 lines
    // lines = lines[0..10].to_vec();
    let mut document = Document::new(lines, 0);

    // Store the diffs from 0..1..2..3..4..5..6..7..8..9
    let mut diffs: Vec<Vec<Diff>> = Vec::new();

    for i in 1..10 {
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

    let mut patches: Vec<Vec<Patch>> = Vec::new();


    for diff in diffs {
        let mut i = 0;
        let mut patch_vec: Vec<Patch> = Vec::new();
        while i < diff.len() {
            // collapse many continguous inserts into a patch
            let mut patch = Patch {
                diff_type: diff[i].diff_type.clone(),
                diffs: Vec::new(),
                index: diff[i].index,
            };

            let mut j = i + 1;
            if diff[i].diff_type == ChangeTag::Insert {                
                while j < diff.len() && diff[j].diff_type == ChangeTag::Insert {
                    j += 1;
                }
                for k in i..j {
                    patch.diffs.push(diff[k].line.clone());
                }
                i = j;
            } else {
                while j < diff.len() && diff[j].diff_type == ChangeTag::Delete {
                    j += 1;
                }
                for _ in i..j {
                    patch.diffs.push(String::from(""));
                }
                i = j;
            }

            patch_vec.push(patch);
        }

        patches.push(patch_vec);
    }

    // Apply patches
    // for patch in &patches {
    //     for p in patch {

    //         // print operation, index, and len() diffs
    //         println!("{:?} {} {}", p.diff_type, p.index, p.diffs.len());

    //         // apply patch
    //         document.apply_patch(p.clone());
    //     }
    //     println!("");
    // }

    // Apply first patch
    for patch in &patches {
        for p in patch {
            document.apply_patch(p.clone());
            // print operation, index, and len() diffs
            println!("{:?} {} {}", p.diff_type, p.index, p.diffs.len());
        }
        println!("");
    }

    document.print(true);

}