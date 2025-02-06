use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use crate::day_two::Report;

pub fn file_to_records(file_path: &str) -> [Report; 1000] {
    let mut reports = core::array::from_fn(|_| Report::default());
    let file = File::open(file_path).expect("Unable to open file!");
    let reader = BufReader::new(file);

    for (i, line) in reader.lines().enumerate() {
        let line = line.expect("Error reading file");

        reports[i].set(line
            .split(" ")
            .map(|level| level.parse::<u8>().expect("Invalid data, expected u8"))
            .collect()
        );
    }

    reports
}