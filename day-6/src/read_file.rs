use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use crate::map::Map;
use crate::location::Location;
use crate::location::Point;

pub fn get_map_and_location(file_path: &str) -> Map {
    let mut map: Map = Map::default();
    let file = File::open(file_path).expect("Unable to open file!");
    let reader = BufReader::new(file);

    for (row_index, line) in reader.lines().enumerate() {
        let line = line.expect("Error reading file");
        
        for (col_index, character) in line.chars().enumerate() {

            map.area[row_index][col_index] = character != '.';

            if character == '^' {
                let starting_position = Point { y: row_index.try_into().unwrap(), x: col_index.try_into().unwrap() };
                let starting_direction = Point { x: 0, y: -1 };
                map.location = Location { position: starting_position, direction: starting_direction };
                map.area[row_index][col_index] = false;
            }
        }
    }

    // println!("{:?}", map);

   map
}
