#[derive(Clone, Debug)]
pub struct Report {
    levels: Vec<u8>,
}

impl Report {
    pub fn set(&mut self, levels: Vec<u8>) {
        self.levels = levels
    }

    pub fn is_safe(&mut self) -> (bool, usize) {
        let mut last: u8 = self.levels[0];
        let ascend: bool = last < self.levels[1];

        for i in 1..self.levels.len() {
            let current = self.levels[i];

            if last > current && ascend || last < current && !ascend || last.abs_diff(current) < 1 || last.abs_diff(current) > 3 {
                return (false, i);
            }

            last = current;
        }

        (true, 0)
    }

    pub fn dampen(&mut self, index: usize) -> &mut Self {
        self.levels.remove(index);
        return self;
    }
}

impl Default for Report {
    fn default() -> Report {
        Report {
            levels: Vec::new()
        }
    }
}