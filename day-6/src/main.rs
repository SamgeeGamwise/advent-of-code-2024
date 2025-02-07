use read_file::EDGE_LENGTH;

mod read_file;

fn main() {
    let (mapped_area, starting_position) = read_file::file_to_2d_array("input.txt");

    let mut distinct_positions: u16 = 0;
    let mut position = starting_position;
    loop {
        if position.0 < 0 || position.1 >= EDGE_LENGTH || position.1 < 0 || position.1 >= EDGE_LENGTH {
            break;
        }
    }

    println!()
}
