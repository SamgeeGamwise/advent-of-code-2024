#[derive(Debug, Clone)]
pub struct Equation {
    pub answer: u64,
    pub numbers: Vec<u64>,
}

impl Equation {
    pub fn new(answer: u64, numbers: Vec<u64>) -> Self {
        Self { answer, numbers }
    }

    pub fn subtract(&mut self, number: u64) {
        self.answer += number;
    }

    pub fn add(&mut self, number: u64) {
        self.answer -= number;
    }

    pub fn multiply(&mut self, number: u64) {
        self.answer /= number;
    }

    pub fn divide(&mut self, number: u64) {
        self.answer *= number;
    }

    pub fn solve(&mut self) -> bool {
        let mut equation = self.clone();
        let last_index = equation.numbers.len() - 1;
        let answer_clone = equation.answer.clone();

        try_operators(&mut equation, last_index, answer_clone)
    }
}

pub fn try_operators(equation: &mut Equation, index: usize, answer: u64) -> bool {
    if index == equation.numbers.len() - 1 {
        equation.answer = answer;
    }

    for operator in 0..4 {
        println!("{:?}", equation.answer);
        println!("{:?}", equation.numbers[index]);

        match operator {
            0 => equation.subtract(equation.numbers[index]),
            1 => equation.add(equation.numbers[index]),
            2 => equation.multiply(equation.numbers[index]),
            3 => equation.divide(equation.numbers[index]),
            _ => panic!("Invalid operator"),
        };
        
        if index == 0 {
            return equation.answer == 0;
        }

        try_operators(equation, index - 1, answer);
    }

    return false;
}