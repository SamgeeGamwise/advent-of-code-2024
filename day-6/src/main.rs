use std::time::Instant;

use read_file::{Location, Point};

mod read_file;
mod day_6;

fn main() {
    let start = Instant::now();
    let (mapped_area, starting_position) = read_file::file_to_2d_array("input.txt");
    let distinct_points = day_6::find_distinct_positions(mapped_area, Location { position: starting_position, direction: Point { x: 0,  y: -1 }});

    println!("{}", distinct_points);
    println!("Time elapsed is: {:?}", start.elapsed());
}

