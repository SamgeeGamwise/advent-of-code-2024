#[derive(Debug, Clone)]
pub struct Equation {
    pub answer: f64,
    pub numbers: Vec<f64>,
}

impl Equation {
    pub fn new(answer: f64, numbers: Vec<f64>) -> Self {
        Self { answer, numbers }
    }

    fn concatenate(&mut self, index: usize) {
        let new_number = format!("{}{}", self.numbers[0], self.numbers[index + 1]).parse::<f64>().unwrap();
        self.numbers[0] = new_number;
        self.numbers.remove(index + 1);
    }

    fn add(&mut self, index: usize) {
        self.numbers[0] += self.numbers[index + 1];
    }

    fn multiply(&mut self, index: usize) {
        self.numbers[0] *= self.numbers[index + 1];
    }

    pub fn check_if_solved(&mut self, flags: &mut Vec<u8>) -> bool {
        for i in 0..flags.len() {
            match flags[i] {
                2 =>  {
                    flags.remove(i);
                    self.concatenate( i);
                },
                _ => continue,
            }
        }
        
        for i in 0..flags.len() {
            match flags[i] {
                0 => self.add(i),
                1 => self.multiply(i),
                _ => continue,
            }
        }

        // println!("{:?}", self.answer);
        // println!("{:?}", self.numbers);
        // println!("{:?}", flags);

        self.answer == self.numbers[0]
    }

    pub fn solve(&mut self) -> bool {
        let mut flags = vec![0; self.numbers.len() - 1];

        loop {
            let mut equation = self.clone();

            if equation.check_if_solved(&mut flags) {
                return true;
            }

            if flags.iter().all(|&x| x == 2) {
                return false;
            }

            for i in 0..flags.len() {
                if flags[i] == 1 {
                    flags[i] = 2;

                    for j in 0..i {
                        flags[j] = 0;
                    }

                    break;
                }

                if flags[i] == 0 {
                    flags[i] = 1;

                    for j in 0..i {
                        flags[j] = 0;
                    }

                    break;
                }
            }
        }
    }
}

// ** 472290821152397