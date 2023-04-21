#[derive(Debug, Clone)]
pub struct Identifier {
    pub position: u8,
    pub site_id: u8
}

impl Eq for Identifier {}
impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.site_id == other.site_id
    }
}

impl Ord for Identifier {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Identifier {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if (self.position < other.position) || (self.position == other.position && self.site_id < other.site_id) {
            Some(std::cmp::Ordering::Less)
        } else {
            Some(std::cmp::Ordering::Greater)
        }
    }
}

#[derive(Debug, Clone)]
pub struct PID {
    pub position: Vec<Identifier>,
    pub logical_clock: u32
}

impl Eq for PID {}
impl PartialEq for PID {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl Ord for PID {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for PID {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let m = other.position.len();
        let n = self.position.len();

        for j in 0..m {
            let mut predecessors_are_fine = true;
            for i in 0..j{
                if self.position[i] != other.position[i] {
                    predecessors_are_fine = false;
                    break;
                }
            }

            let mut ordering_condition = false;
            if predecessors_are_fine {
                ordering_condition = j == n + 1 || self.position[j] < other.position[j];
            }
            
            if predecessors_are_fine && ordering_condition {
                return Some(std::cmp::Ordering::Less);
            }
        }

        return Some(std::cmp::Ordering::Greater);
        
        // for j in 0..n {
        //     let mut predecessors_are_fine = true;
        //     for i in 0..j{
        //         if other.position[i] != self.position[i] {
        //             predecessors_are_fine = false;
        //             break;
        //         }
        //     }

        //     let mut ordering_condition = false;
        //     if predecessors_are_fine {
        //         ordering_condition = j == m + 1 || other.position[j] < self.position[j];
        //     }
            
        //     if predecessors_are_fine && ordering_condition {
        //         return Some(std::cmp::Ordering::Greater);
        //     }
        // }

        // panic!("Found a causal ordering that did not make sense in partial_cmp for PID");
        //  Some(std::cmp::Ordering::Equal)
    }
}

impl PID {
    pub fn create_start() -> PID {
        PID {
            position: vec![Identifier { position: 0, site_id: 0 }],
            logical_clock: 0
        }
    }

    pub fn create_end() -> PID {
        PID {
            position: vec![Identifier { position: 255, site_id: 255 }],
            logical_clock: 0
        }
    }

    pub fn is_start(&self) -> bool {
        self.position[0].position == 0 && self.position[0].site_id == 0
    }

    pub fn is_end(&self) -> bool {
        self.position[0].position == 255 && self.position[0].site_id == 255
    }
}