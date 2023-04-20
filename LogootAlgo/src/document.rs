use crate::{uuid::{PID, Identifier}, Patch};
use similar::{ChangeTag};
use rand::Rng;
use convert_base::Convert;

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
pub struct Line {
    identifier: PID,
    content: String,
}


impl Ord for Line {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.identifier.cmp(&other.identifier)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Document {
    lines: Vec<Line>,
    site: u8,
}

impl Line {
    pub fn pos_len(&self) -> usize {
        self.identifier.position.len()
    }
}

impl Document {
    pub fn new(lines: Vec<String>, site: u8) -> Document {
        let mut document = Document { lines: Vec::new() , site};
        
        // Create start line
        let start_identifier = PID::create_start();
        let start_line = Line { identifier: start_identifier, content: String::from("") }; // Empty content since this is the start line

        // Create end line
        let end_identifier = PID::create_end();
        let end_line = Line { identifier: end_identifier, content: String::from("") }; // Empty content since this is the start line
        document.lines.push(start_line);
        document.lines.push(end_line);

        document.insert(0, lines, site);

        document
    }

    pub fn insert(&mut self, insert_index : usize, lines: Vec<String>, site: u8) {
        let mut insert_index = insert_index;
        // Insert beginning lines
        let mut pids : Vec<PID> = Self::generate_line_positions(self, insert_index, insert_index+1, lines.len() as u32, site);
        // println!("pids: {:?}", pids.len());
        // println!("lines: {:?}", lines.len());

        for line in lines {
            let pid = pids.remove(0);
            // println!("pid: {:?}", pid);
            let new_line = Line { identifier: pid, content: line };
            self.lines.insert(insert_index, new_line);
            insert_index += 1;
        }
    }

    pub fn remove(&mut self, remove_index : usize, num_lines: usize) {
        for _ in 0..num_lines {
            self.lines.remove(remove_index);
        }
    }

    pub fn apply_patch(&mut self, patch : Patch) {
        if patch.diff_type == ChangeTag::Delete {
            self.remove(patch.index, patch.diffs.len());
        } else if patch.diff_type == ChangeTag::Insert {
            self.insert(patch.index, patch.diffs, self.site);
        }
    }


    pub fn print(&self, save: bool) {
        
        let mut s = String::new();
        for i in 0..self.lines.len() {
            let line = &self.lines[i];
            if line.identifier.is_start() || line.identifier.is_end() {
                continue;
            }
            s.push_str(&line.content.trim());
            if i != self.lines.len() - 3 {
                s.push_str("\n");
            }
        }

        if save {
            std::fs::write("output.txt", s).expect("Unable to write file");
        } else {
            println!("{}", s);
        }
    }


    pub fn prefix(position: &Line, index: u32)  -> u32 {
        let mut prefix = String::new();
        for i in 0..index {
            // prefix += position.identifier.position[i as usize].position as u32;
            // append the position as a string
            // check if index is out of bounds
            if i as usize >= position.identifier.position.len() {
                // fill with zero
                prefix.push('0');
            } else {
                prefix.push_str(&position.identifier.position[i as usize].position.to_string());
            }
        }
        
        // convert the prefix to a u32 in base 256 manually
        let prefix = prefix.as_bytes();
        let prefix = u32::from_str_radix(&String::from_utf8_lossy(prefix), 10).unwrap();
        prefix
        
    }

    pub fn construct_position(r: &mut u32, p: &Line, q: &Line, s: u8) -> PID {
        let mut list: Vec<Identifier> = Vec::new();
        let r = r.clone();
        // store each digit of r in a vector oo u8
        let string: String = r.to_string();

        let mut r = Vec::<u8>::new();
        for c in string.chars() {
            r.push(c.to_digit(10).unwrap() as u8);
        }

        r.reverse();
        let mut base = Convert::new(10,256);
        let mut r = base.convert::<u8,u8> (&r);
        r.reverse();

        // r: convert 14 -> 00001110
        // let r = format!("{:08b}", r);


        // constructs a position <<r1, s1>, <r2, s2>,.....<hn, sn>> where ri is the ith digit of r.
        // We use the following rules to define each si : 
        // 1) if i = n then si = s, 
        // 2) else if ri = pi.pos then si = pi.site, 
        // 3) else if ri = qi.pos then si = qi.site 
        // 4) else si = s

        for i in 0..r.len() {
            let ri = r[i] as u8;

            if i == r.len() - 1 {
                list.push(Identifier { position: ri, site_id: s });
            } else if i < p.identifier.position.len() && ri == p.identifier.position[i].position {
                list.push(Identifier { position: ri, site_id: p.identifier.position[i].site_id });
            } else if i < q.identifier.position.len() && ri == q.identifier.position[i].position {
                list.push(Identifier { position: ri, site_id: q.identifier.position[i].site_id });
            } else {
                list.push(Identifier { position: ri, site_id: s });
            }
        }


        PID {
            position: list,
            logical_clock: 0,
        }
    }

    pub fn generate_line_positions(&mut self, p_ind: usize, q_ind: usize, n: u32, s: u8) -> Vec<PID> {
        let p: &Line = &self.lines[p_ind];
        let q: &Line = &self.lines[q_ind];

        let mut index = 0;
        let mut interval = 0;

        while interval < n {
            index += 1;
            let q_val = Self::prefix(q, index);
            let p_val = Self::prefix(p, index);
            interval = q_val - p_val;
        }

        // let mut list: PID = Vec::new();
        let mut list: Vec<PID> = Vec::new();

        let step = interval / n;
        let mut r = Self::prefix(p, index);
        for _ in 0..n {
            // let rand = random(1, step);
            let mut rng = rand::thread_rng();
            let rand = if step !=1 { rng.gen_range(1..step) } else { 1 };
            list.push(Self::construct_position(&mut (r + rand),p,q,s));
            r += step;
        }

        list
    }

    pub fn insert_identifier(&mut self, line : Line) {
        // Convert to a binary search version
        let mut index = 0;
        let mut size = self.lines.len();
        while size > 0 {
            let half = size / 2;
            let mid = index + half;
            let line2 = &self.lines[mid];
            if line.identifier < line2.identifier {
                index = mid + 1;
                size -= half + 1;
            } else {
                size = half;
            }
        }
        self.lines.insert(index, line);
    }

    
    
    pub fn remove_identifier(&mut self, identifier: &PID) {
        // let mut index = 0;
        // for line in &self.lines {
        //     if line.identifier == *identifier {
        //         self.lines.remove(index);
        //         break;
        //     }
        //     index += 1;
        // }
        // Convert to a binary search version
        let mut index = 0;
        let mut size = self.lines.len();
        while size > 0 {
            let half = size / 2;
            let mid = index + half;
            let line = &self.lines[mid];
            if line.identifier == *identifier {
                self.lines.remove(mid);
                break;
            } else if line.identifier < *identifier {
                index = mid + 1;
                size -= half + 1;
            } else {
                size = half;
            }
        }
    }

}