use std::{fs::{metadata, File}, io::{BufRead, BufReader}};

pub fn read_txt_to_array() {
    let file_path = "numbers.txt";
    let file = File::open(file_path).expect("Unable to open file!");
    let reader = BufReader::new(file);

    let mut values1: Vec<u16> = Vec::new();
    let mut values2: Vec<u16> = Vec::new();

    for line in reader.lines(){
        let line = line.expect("Error reading file");
        values1.push(line[..5].parse::<u16>().expect("Could not parse to u16"));
        values2.push(line[8..].parse::<u16>().expect("Could not parse to u16"));
    }
}