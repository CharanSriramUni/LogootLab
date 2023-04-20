use crate::{uuid::{PID, Identifier}, Line};
use rand::Rng;

fn random(x: u8, y: u8) -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(x + 1..y)
}

impl Ord for Line {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.identifier.cmp(&other.identifier)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Document {
    lines: Vec<Line>,
}

impl Line {
    pub fn pos_len(&self) -> usize {
        self.identifier.position.len()
    }
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
        document.lines.push(end_line);

        let s = &document.lines[0];

        // Insert beginning lines
        for line in lines {
            
        }

        document
    }

    pub fn generate_between(left: &Line, right: &Line, site: u8) -> Option<Vec<Identifier>> {
        if left < right {
            return None;    
        } 

        let left = &left.identifier.position;
        let right = &right.identifier.position;
        
        let mut new_pos = Vec::<Identifier>::new();
        for i in 0..left.len() {
            let l = &left[i];
            let r = &right[i];

            if l.position == r.position && l.site_id == r.site_id {
                new_pos.push(Identifier { position: l.position, site_id: l.site_id });
                continue;
            }

            let d = r.position - l.position;
            if d > 1 {
                let r  = random(l.position, r.position);
                new_pos.push(Identifier { position: r, site_id: site });
            } else if d == 1 {
                if site < l.site_id {
                    new_pos.push(Identifier { position: l.position, site_id: site });
                } else if site > l.site_id {
                    new_pos.push(Identifier { position: r.position, site_id: site });
                } else {
                    let mut min: u8 = 0;
                    if left.len() > right.len() {
                        min = left[right.len()].position;

                        if min == u8::MAX - 1 {
                            let r = random(0, u8::MAX);
                            new_pos.push(Identifier { position: l.position, site_id: l.site_id });
                            new_pos.extend_from_slice(&left[right.len()..]);
                            new_pos.push(Identifier { position: r, site_id: site });
                        }
                    }
                    let r = random(0, u8::MAX);
                    new_pos.push(Identifier { position: l.position, site_id: l.site_id });
                    new_pos.push(Identifier { position: r, site_id: site });
                }
            } else {
                if site > l.site_id && site < r.site_id {
                    new_pos.push(Identifier { position: l.position, site_id: site });
                } else {
                    let r = random(0, u8::MAX);
                    new_pos.push(Identifier { position: l.position, site_id: l.site_id });
                    new_pos.push(Identifier { position: r, site_id: site });
                }
            }
            return Some(new_pos);
        }

        if right.len() > left.len() {
            let r = random(0, right[left.len()].position);
            new_pos.push(Identifier { position: r, site_id: site });
        }

        return Some(new_pos)
    }

    // pub fn prefix(position: &Line, index: u32) {
        
    // }

    // pub fn generate_line_positions(&mut self, p_ind: usize, q_ind: usize, lines: Vec<String>, s: u8) {
    //     let p = &self.lines[p_ind];
    //     let q = &self.lines[q_ind];
    //     let N = lines.len() as u32;

    //     let mut index = 0;
    //     let mut interval = 0;

    //     while interval < N {
    //         index += 1;
            
    //     }
    // }

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