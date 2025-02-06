const ROWS: usize = 140;
const COLS: usize = 140;

pub fn count_all_xmas(word_search: [[char; COLS]; ROWS]) -> u32 {
    let mut xmas_count: u32 = 0;
    // Left, Right, Down, Up, Down/Left, Down/Right, Up/Left, Up/Right
    let vectors: Vec<(i8, i8)> = vec![
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];

    for row_index in 0..word_search.len() {
        for col_index in 0..word_search[row_index].len() {
            for vector in &vectors {
                xmas_count += check_for_xmas(
                    word_search, 
                    (row_index.try_into().unwrap(), col_index.try_into().unwrap()), 
                    *vector
                );
            }
        }
    }

    xmas_count
}

fn check_for_xmas(word_search: [[char; COLS]; ROWS], position: (i64, i64), vector: (i8, i8)) -> u32 {
    let word_length: i8 = 4;
    let endpoint = (position.0 + ((word_length - 1) * vector.0) as i64, position.1 + ((word_length - 1) * vector.1) as i64);

    if endpoint.0 < 0 || endpoint.0 >= ROWS as i64 || endpoint.1 < 0 || endpoint.1 >= COLS as i64 {
        return 0;
    } 

    let mut possible_xmas: String = "".to_string();

    for i in 0..word_length {
        let row_index = usize::try_from(position.0 + (i * vector.0) as i64).expect("Invalid character!");
        let col_index = usize::try_from(position.1 + (i * vector.1) as i64).expect("Invalid character!");

        possible_xmas.push(word_search[row_index][col_index]);
    }

    if possible_xmas == "XMAS"{
        return 1;
    }

    return 0;
}