use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub fn file_to_sorted_array(file_path: &str) -> ([u32; 1000], [u32; 1000]) {
    let file = File::open(file_path).expect("Unable to open file!");
    let reader = BufReader::new(file);

    let mut values1: [u32; 1000] = [0; 1000];
    let mut values2: [u32; 1000] = [0; 1000];

    for (i, line) in reader.lines().enumerate() {
        let line = line.expect("Error reading file");
        values1[i] = line[..5].parse::<u32>().expect("Could not parse to u16");
        values2[i] = line[8..].parse::<u32>().expect("Could not parse to u16");
    }

    values1.sort_unstable();
    values2.sort_unstable();


    (values1, values2)
}
