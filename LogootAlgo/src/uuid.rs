#[derive(Debug, Clone)]
pub struct Identifier {
    pub position: u8,
    pub site_id: u8
}

#[derive(Debug, Clone)]
pub struct PID {
    pub site_id: Vec<Identifier>,
    pub logical_clock: u32
}

impl Eq for PID {}
impl PartialEq for PID {
    fn eq(&self, other: &Self) -> bool {
        self.site_id == other.site_id
    }
}

impl Ord for PID {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for PID {
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

impl PID {
    pub fn create_start() -> PID {
        let mut site_id = Vec::new();
        site_id.push(0);
        PID { site_id }
    }

    pub fn create_end() -> PID {
        let mut site_id = Vec::new();
        site_id.push(1);
        PID { site_id }
    }

    pub fn generate_between(&self, other: &PID) -> PID {
        let mut new_site_id: Vec<u8>;

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
            panic!("Cannot generate a PID between two equal PIDs")
        }

        PID {
            site_id: new_site_id
        }
    }
}