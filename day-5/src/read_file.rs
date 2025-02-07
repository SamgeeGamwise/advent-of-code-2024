use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Debug)]
pub struct Rule {
    pub before: u8,
    pub after: u8
}

impl Rule {
    pub fn new(before: u8, after: u8) -> Rule {
        Rule {
            before,
            after,
        }
    }
}

pub fn file_to_rules(file_path: &str) -> Vec<Rule> {
    let mut rules: Vec<Rule> = Vec::new();

    let file = File::open(file_path).expect("Unable to open file!");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Error reading file");
        rules.push(Rule::new(line[..2].parse::<u8>().unwrap(), line[3..].parse::<u8>().unwrap()));
    }

    rules
}