use std::collections::HashSet;
use location::Location;
use time_elapsed::log_time_elapsed;

mod read_file;
mod map;
mod location;

#[log_time_elapsed]
fn main() {
    let map = read_file::get_map_and_location("input.txt");

    let visited_locations = map.get_locations();
    let mut trapped_count = 0;

    // Remove duplicate Points from visited_locations
    let mut unique_locations = HashSet::new();
    let visited_locations: Vec<Location> = visited_locations
        .into_iter()
        .filter(|location| unique_locations.insert(location.position)) // Inserts only if new
        .collect();

    for blockage in &visited_locations {
        if !map.able_to_leave(blockage) {
            trapped_count += 1;
        }
    }

    println!("Trapped Count: {}", trapped_count);  
}