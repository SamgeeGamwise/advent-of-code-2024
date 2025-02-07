use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub const RULES_LENGTH: usize = 1176;
pub const PAGES_LENGTH: usize = 193;

pub fn file_to_rules(file_path: &str) -> [(u8, u8); RULES_LENGTH] {
    let mut rules: [(u8, u8); RULES_LENGTH] = [Default::default(); RULES_LENGTH];

    let file = File::open(file_path).expect("Unable to open file!");
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.expect("Error reading file");
        rules[index] = (line[..2].parse::<u8>().unwrap(), line[3..].parse::<u8>().unwrap());
    }

    rules
}

pub fn file_to_pages(file_path: &str) -> [Vec<u8>; PAGES_LENGTH] {
    let mut pages: [Vec<u8>; PAGES_LENGTH] = std::array::from_fn(|_| Vec::new());

    let file = File::open(file_path).expect("Unable to open file!");
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.expect("Error reading file");
        pages[index] = line.split(",").map(|number| number.parse::<u8>().unwrap()).collect();
    }

    pages
}