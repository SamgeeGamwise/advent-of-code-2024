use crate::read_file::Point;
use crate::read_file::EDGE_LENGTH;

pub fn find_distinct_positions(mapped_area: [[bool; EDGE_LENGTH]; EDGE_LENGTH], starting_position: Point) -> Vec<Point> {
    let mut distinct_points: Vec<Point> = Vec::new();
    
    let success = walk(mapped_area, starting_position, &mut distinct_points);

    println!("{}", success);

    distinct_points
}

pub fn walk(mapped_area: [[bool; EDGE_LENGTH]; EDGE_LENGTH], starting_position: Point, distinct_points: &mut Vec<Point>) -> bool {
    let mut count = 0;
    let mut position = starting_position;
    let mut direction = Point { x: 0, y: -1 };

    distinct_points.push(starting_position);

    loop {
        position += direction;


        if position.x < 0 || position.x >= EDGE_LENGTH.try_into().unwrap() || position.y < 0 || position.y >= EDGE_LENGTH.try_into().unwrap() {
            return true;
        }

        
        if mapped_area[position.y as usize][position.x as usize] {
            position -= direction;
            direction.turn();
        } else {
            if !distinct_points.contains(&position) {
                println!("{:?}", position);
                distinct_points.push(position);
                count = 0;
            } else {
                count += 1;
            }
        }

        if count > 10 {
            return false;
        }
    }
}