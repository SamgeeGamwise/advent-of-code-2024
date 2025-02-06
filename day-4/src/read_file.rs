use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub fn file_to_2d_array(file_path: &str) -> [[char; 140];140] {
    let mut word_search: [[char; 140]; 140] = [[char::default(); 140]; 140];

    let file = File::open(file_path).expect("Unable to open file!");
    let reader = BufReader::new(file);

    for (line_index, line) in reader.lines().enumerate() {
        let line = line.expect("Error reading file");
        line
            .chars()
            .enumerate()
            .for_each(|(character_index, character)| word_search[line_index][character_index] = character);
    }

    word_search
}