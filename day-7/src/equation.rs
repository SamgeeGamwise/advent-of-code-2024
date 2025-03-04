#[derive(Debug, Clone)]
pub struct Equation {
    pub answer: f64,
    pub numbers: Vec<f64>,
}

impl Equation {
    pub fn new(answer: f64, numbers: Vec<f64>) -> Self {
        Self { answer, numbers }
    }

    pub fn concatenate(&mut self, index: usize, flags: &mut Vec<u8>) {

        if index == self.numbers.len() - 1 {
            return;
        }

        let number = self.numbers[index];
        let next_number = self.numbers[index + 1];

        let new_number = format!("{}{}", number, next_number).parse::<f64>().unwrap();

        self.numbers.remove(index);
        flags.remove(index);
        self.numbers[index] = new_number;
    }

    pub fn add(&mut self, number: f64) {
        self.answer -= number;
    }

    pub fn multiply(&mut self, number: f64) {
        self.answer /= number;
    }

    pub fn solve(&mut self) -> bool {
        let mut flags = vec![0; self.numbers.len()];
        
        loop {
            let mut equation = self.clone();
            
            // println!("{:?}", flags);
            if equation.check_if_solved(&mut flags) {
                // println!("{:?}", self.answer);
                return true;
            }
           
            if flags[0..flags.len() - 1].iter().all(|&x| x == 1) {
                return false;
            }

            for i in 0..flags.len() - 1 {
                // if flags[i] == 1 {
                //     flags[i] = 2;

                //     for j in (0..i).rev() {
                //         flags[j] = 0;
                //     }

                //     break;
                // }

                if flags[i] == 0 {
                    flags[i] = 1;

                    for j in (0..i).rev() {
                        flags[j] = 0;
                    }

                    break;
                }
            }

        }
    }

    fn check_if_solved(&mut self, flags: &mut Vec<u8>) -> bool {
        for i in (0..flags.len()).rev() {
            match flags[i] {
                2 =>  self.concatenate(i, flags),
                _ => continue,
            }
        }
        
        for i in (0..flags.len()).rev() {
            match flags[i] {
                0 => self.add(self.numbers[i]),
                1 => self.multiply(self.numbers[i]),
                // 2 =>  self.concatenate(i),
                _ => continue,
            }

            // println!("{:?}", self.answer);
        }

        // if self.answer == 0.0 {
            // let mut equation_string = String::new();

            // for i in (0..self.numbers.len()).rev() {
            //     match flags[i] {
            //         0 => equation_string.push_str(&format!(" - {}", self.numbers[i])),
            //         1 => equation_string.push_str(&format!(" / {}", self.numbers[i])),
            //         2 => equation_string.push_str(&format!(" {:?}", self.numbers)),
            //         _ => continue,
            //     }
            // }

            // println!("{}", equation_string);
        // }

        self.answer == 0.0
    }
}


// 5540634308465
// 5540634308465
// 37936001450
// 727919199967100
// ** 472290821152397
// Off by 103