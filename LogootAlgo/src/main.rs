pub mod uuid;
pub mod document;
use document::Document;
use std::{fs::File};
use std::io::prelude::*;
use std::env;
use similar::{ChangeTag, TextDiff};


// structures for storing diffs
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

// take in stdargs
fn main() {
    let args: Vec<String> = env::args().collect();
    // -n is number of revisions
    // -s is number of sentences included
    let mut n: u32 = 20;
    let mut s: u32 = 20;
    let mut path = "../DataProcessing/text";

    for i in 1..args.len() {
        if args[i] == "-n" {
            n = args[i+1].parse::<u32>().unwrap();
        }
        if args[i] == "-s" {
            s = args[i+1].parse::<u32>().unwrap();
        }
        if args[i] == "-f" {
            path = &args[i+1];
        }
    }

    // 100 files in ../data with versions of a Wikipedia article

    // Read in the first version ../data/0.txt as a list of strings
    let og = format!("{}/{}.txt", path, 0);
    println!("{}", og);
    let mut file = File::open(og).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("something went wrong reading the file");

    // Split by new line and add to array of strings
    let mut lines: Vec<String> = Vec::new();
    // for i in 0..s {
    //     let line = contents.lines().nth(i as usize).unwrap();
    //     lines.push(line.to_string());
    // }

    for line in contents.lines() {
        if s == 0 {
            break;
        }
        lines.push(line.to_string());
        s -= 1;
    }

    // Create a document with the lines
    let mut document = Document::new(lines, 0);

    // Store the diffs from 0..1..n
    let mut diffs: Vec<Vec<Diff>> = Vec::new();

    for i in 1..n {
        // Read in the next version ../data/i.txt as a list of strings
        let path = format!("{}/{}.txt", path, i);
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

    // print diffs
    // for diff in &diffs {
    //     for d in diff {
    //     // print diff type and index
    //         println!("{:?} {:?}", d.diff_type, d.index);
    //     }
    //     println!("");
    // }    


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
    for i in 0..n-1 {
        let patch = patches[i as usize].clone();
        document.apply_patches(&patch, false);
    }


    // Print size of text
    println!("LOGOOT SIZE: {}", document.get_logoot_size());
    println!("ARTICLE SIZE: {}", document.get_article_size());
}