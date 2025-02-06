pub struct Report {
    levels: Vec<u8>,
}

impl Report {
    pub fn set(&mut self, levels: Vec<u8>) {
        self.levels = levels
    }

    pub fn is_safe(&mut self) -> bool {
        let mut last: u8 = self.levels[0];
        let ascend: bool = last < self.levels[1];

        for i in 1..self.levels.len() {
            let current = self.levels[i];

            if last > current && ascend || last < current && !ascend || last.abs_diff(current) < 1 || last.abs_diff(current) > 3 {
                return false;
            }

            last = current;
        }

        true
    }

    pub fn dampened_safe(&mut self) -> bool {
        for i in 0..self.levels.len() {
            let removed = self.levels.remove(i);
    
            if self.is_safe() {
                return true;
            }

            self.levels.insert(i, removed);
        }

        false
    }
}

impl Default for Report {
    fn default() -> Report {
        Report {
            levels: Vec::new()
        }
    }
}