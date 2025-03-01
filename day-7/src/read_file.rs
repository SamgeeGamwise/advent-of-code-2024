use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use crate::equation::Equation;

pub fn read_file(file_path: &str) -> Vec<Equation> {

    let file = File::open(file_path).expect("Unable to open file!");
    let reader = BufReader::new(file);
    let mut equations: Vec<Equation> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Error reading file");
        let split = line.find(":").expect("Invalid input!");
        let answer = line[..split].parse::<f64>().expect("Expected a number!");

        // Reverse the order of the numbers while parsing
        let numbers: Vec<f64> = line[split+2..]
        .split(" ")
        .map(|x| x.trim().parse().unwrap())
        .collect::<Vec<f64>>(); // Collect first

        equations.push(Equation::new(answer, numbers));
    }

    equations
}