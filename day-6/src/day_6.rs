use crate::read_file::Point;
use crate::read_file::EDGE_LENGTH;

pub fn find_distinct_positions(mut mapped_area: [[bool; EDGE_LENGTH]; EDGE_LENGTH], starting_position: Point) -> u32 {
    let mut distinct_points: Vec<(Point, Point)> = Vec::new();
    
    walk(mapped_area, starting_position, Point { x: 0, y: -1 }, &mut distinct_points);

    let mut trapped_count = 0;

    for i in 2..distinct_points.len() {
        mapped_area[distinct_points[i].0.x as usize][distinct_points[i].0.y as usize] = true;

        if !walk(mapped_area, distinct_points[i].0, distinct_points[i].1, &mut Vec::new()) {
            trapped_count += 1;
            println!("{}", trapped_count);
        }

        mapped_area[distinct_points[i].0.x as usize][distinct_points[i].0.y as usize] = false;
    }

    trapped_count
}

pub fn walk(mapped_area: [[bool; EDGE_LENGTH]; EDGE_LENGTH], starting_position: Point, starting_direction: Point, distinct_points: &mut Vec<(Point, Point)>) -> bool {
    let mut position = starting_position;
    let mut direction = starting_direction;

    distinct_points.push((starting_position, direction));

    loop {
        position += direction;


        if position.x < 0 || position.x >= EDGE_LENGTH.try_into().unwrap() || position.y < 0 || position.y >= EDGE_LENGTH.try_into().unwrap() {
            return true;
        }

        
        if mapped_area[position.y as usize][position.x as usize] {
            position -= direction;
            direction.turn();
        } else {
            let mut distinct: bool = true;
            let mut repeat: bool = false;

            for i in 0..distinct_points.len() {
                if distinct_points[i].0 == position {
                    distinct = false;

                    if distinct_points[i].1 == direction {
                        repeat = true;
                    }
                }
            }

            if repeat {
                return false;
            }

            if distinct {
                distinct_points.push((position, direction));
            }
        }
    }
}