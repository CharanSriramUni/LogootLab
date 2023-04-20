#[derive(Debug)]
pub struct UUID {    
    pub site_id: Vec<u8>,
}

impl Eq for UUID {}
impl PartialEq for UUID {
    fn eq(&self, other: &Self) -> bool {
        self.site_id == other.site_id
    }
}

impl PartialOrd for UUID {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            Some(std::cmp::Ordering::Equal)
        } else {
            let mut i = 0;
            while i < self.site_id.len() && i < other.site_id.len() {
                if self.site_id[i] < other.site_id[i] {
                    return Some(std::cmp::Ordering::Less);
                } else if self.site_id[i] > other.site_id[i] {
                    return Some(std::cmp::Ordering::Greater);
                }
                i += 1;
            }

            if self.site_id.len() < other.site_id.len() {
                Some(std::cmp::Ordering::Less)
            } else if self.site_id.len() > other.site_id.len() {
                Some(std::cmp::Ordering::Greater)
            } else {
                // Equal lengths and contents
                Some(std::cmp::Ordering::Equal)
            }
        }
    }
}

pub fn lsb(any: &UUID) -> u8 {
    any.site_id[any.site_id.len() - 1]
}

pub fn tuple_average(a: u8, b: u8) -> (u8, u8) {
    let v = ((a as f32 + b as f32) / 2.0) * 10.0;
    let v = v as u8;
    
    let value_1 = v / 10;
    let value_2 = v % 10;
    (value_1, value_2)
}

impl UUID {
    pub fn generate_between(&self, other: &UUID) -> UUID {
        let mut new_site_id = Vec::new();

        if self.site_id.len() == other.site_id.len() {
            let (v1, v2) = tuple_average(lsb(&self), lsb(&other));
            new_site_id = self.site_id.clone();
            new_site_id.push(v1);
            new_site_id.push(v2);
        }

        if self < other {
            if self.site_id.len() < other.site_id.len() {
                let (v1, v2) = tuple_average(lsb(&other), 0);
                new_site_id = self.site_id.clone();
                new_site_id.push(v1);
                if v2 != 0 {
                    new_site_id.push(v2);
                }
            } else {
                let (v1, v2) = tuple_average(lsb(&self), 10);
                new_site_id = self.site_id.clone();
                new_site_id.pop(); // Remove the top byte
                new_site_id.push(v1);
                if v2 != 0 {
                    new_site_id.push(v2);
                }
            }
        } else if self > other {
            if self.site_id.len() < other.site_id.len() {
                let (v1, v2) = tuple_average(lsb(&other), 10);
                new_site_id = other.site_id.clone();
                new_site_id.pop(); 
                new_site_id.push(v1);
                if v2 != 0 {
                    new_site_id.push(v2);
                }
            } else {
                let (v1, v2) = tuple_average(lsb(&self), 0);
                new_site_id = other.site_id.clone();
                new_site_id.push(v1);
                if v2 != 0 {
                    new_site_id.push(v2);
                }
            }
        } else {
            panic!("Cannot generate a UUID between two equal UUIDs")
        }

        UUID {
            site_id: new_site_id
        }
    }
}