#[derive(Debug, Clone)]
pub struct Equation {
    pub answer: f64,
    pub numbers: Vec<f64>,
}

impl Equation {
    pub fn new(answer: f64, numbers: Vec<f64>) -> Self {
        Self { answer, numbers }
    }

    pub fn subtract(&mut self, number: f64) {
        self.answer += number;
    }

    pub fn add(&mut self, number: f64) {
        self.answer -= number;
    }

    pub fn multiply(&mut self, number: f64) {
        self.answer /= number;
    }

    pub fn divide(&mut self, number: f64) {
        self.answer *= number;
    }

    pub fn solve(&mut self) -> bool {
        let mut flags = vec![0; self.numbers.len()];
        
        loop {
            let mut equation = self.clone();

            if equation.check_if_solved(&flags) {
                // println!("{:?}", self.answer);
                return true;
            }
            
            if flags.iter().all(|&x| x == 1) {
                return false;
            }

            for i in 1..flags.len() {
                if flags[i] == 0 {
                    flags[i] = 1;

                    for j in (1..i).rev() {
                        flags[j] = 0;
                    }

                    break;
                }
            }
        }
    }

    fn check_if_solved(&mut self, flags: &Vec<u8>) -> bool {
        for i in (0..flags.len()).rev() {
            match flags[i] {
                0 => self.add(self.numbers[i]),
                1 => self.multiply(self.numbers[i]),
                2 => self.subtract(self.numbers[i]),
                3 => self.divide(self.numbers[i]),
                _ => continue,
            }
        }

        if self.answer == 0.0 {
            let mut equation_string = String::new();

            for i in (0..self.numbers.len()).rev() {
                match flags[i] {
                    0 => equation_string.push_str(&format!(" - {}", self.numbers[i])),
                    1 => equation_string.push_str(&format!(" / {}", self.numbers[i])),
                    2 => equation_string.push_str(&format!(" - {}", self.numbers[i])),
                    3 => equation_string.push_str(&format!(" / {}", self.numbers[i])),
                    _ => continue,
                }
            }

            println!("{}", equation_string);
        }

        self.answer == 0.0
    }
}


// 5540634308465
// 5540634308465

// Off by 103