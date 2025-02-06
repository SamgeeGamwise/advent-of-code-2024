use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub fn file_to_records(file_path: &str) -> String {
    let mut memory: String = String::new();
    let file = File::open(file_path).expect("Unable to open file!");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Error reading file");
        memory.push_str(&line);
    }

    memory
}