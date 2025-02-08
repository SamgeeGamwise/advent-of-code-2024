use crate::read_file::Location;
use crate::read_file::EDGE_LENGTH;

pub fn find_distinct_positions(mut mapped_area: [[bool; EDGE_LENGTH]; EDGE_LENGTH], starting_location: Location) -> u32 {
    let mut distinct_locations: Vec<Location> = Vec::new();
    
    walk(mapped_area, starting_location, &mut distinct_locations);

    let mut trapped_count = 0;

    for i in 1..distinct_locations.len() {

        // for j in 0..34 {
        //     if distinct_locations[i].position == distinct_locations[j].position {
        //         continue 'outer;
        //     }
        // }

        mapped_area[distinct_locations[i].position.x as usize][distinct_locations[i].position.y as usize] = true;

        if walk(mapped_area, distinct_locations[i-1], &mut Vec::new()) {
            trapped_count += 1;
            println!("{}", trapped_count);
        }

        mapped_area[distinct_locations[i].position.x as usize][distinct_locations[i].position.y as usize] = false;
    }

    trapped_count
}

pub fn walk(mapped_area: [[bool; EDGE_LENGTH]; EDGE_LENGTH], starting_location: Location, distinct_locations: &mut Vec<Location>) -> bool {
    let mut guard_location = starting_location;
    let stuck = true;

    distinct_locations.push(starting_location);

    loop {
        guard_location.step();

        if  guard_location.position.x < 0 || 
            guard_location.position.x >= EDGE_LENGTH.try_into().unwrap() || 
            guard_location.position.y < 0 || 
            guard_location.position.y >= EDGE_LENGTH.try_into().unwrap() {

            return !stuck;
        }

        
        if mapped_area[guard_location.position.y as usize][guard_location.position.x as usize] {
            guard_location.step_back();
            guard_location.turn();
        } else {
            let mut distinct = true;

            for i in 0..distinct_locations.len() {
                if distinct_locations[i] == guard_location  {
                    return stuck;
                }

                if distinct_locations[i].position == guard_location.position {
                    distinct = false;
                }
            }

            if distinct {
                distinct_locations.push(guard_location);
            }
        }
    }
}


// 273
// 696
// 683