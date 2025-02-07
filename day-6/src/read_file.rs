use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub const EDGE_LENGTH: usize = 130;

pub fn file_to_2d_array(file_path: &str) -> ([[bool; EDGE_LENGTH]; EDGE_LENGTH], (usize, usize)) {
    let mut mapped_area: [[bool; EDGE_LENGTH]; EDGE_LENGTH] = [[false; EDGE_LENGTH]; EDGE_LENGTH];
    let mut starting_position: (usize, usize) = (0,0);

    let file = File::open(file_path).expect("Unable to open file!");
    let reader = BufReader::new(file);

    for (row_index, line) in reader.lines().enumerate() {
        let line = line.expect("Error reading file");
        for (col_index, character) in line.chars().enumerate() {
            mapped_area[row_index][col_index] = character != '.';

            if character == '^' {
                starting_position = (row_index, col_index);
            }
        }
    }

    (mapped_area, starting_position)
}