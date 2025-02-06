const ROWS: usize = 140;
const COLS: usize = 140;

pub fn count_all_xmas(word_search: [[char; COLS]; ROWS]) -> u32 {
    let mut xmas_count: u32 = 0;

    for row_index in 0..word_search.len() {
        for col_index in 0..word_search[row_index].len() {
            xmas_count += check_for_xmas(
                word_search, 
                (row_index.try_into().unwrap(), col_index.try_into().unwrap())
            );
        }
    }

    xmas_count
}

fn check_for_xmas(word_search: [[char; COLS]; ROWS], position: (i64, i64)) -> u32 {
    if position.0 - 1 < 0 || position.0 + 1 >= ROWS as i64 || position.1 - 1 < 0 || position.1 + 1 >= COLS as i64 {
        return 0;
    } 

    let mut first_mas: String = "".to_string();
    let mut second_mas: String = "".to_string();
    
    for i in -1..2 {
        let first_mas_row_index = usize::try_from(position.0 + i).expect("Invalid character!");
        let first_mas_col_index = usize::try_from(position.1 - i).expect("Invalid character!");
        let second_mas_row_index = usize::try_from(position.0 + i).expect("Invalid character!");
        let second_mas_col_index = usize::try_from(position.1 + i).expect("Invalid character!");

        println!("First: {:?}", (first_mas_row_index, first_mas_col_index));
        println!("Second: {:?}", (second_mas_row_index, second_mas_col_index));
        

        first_mas.push(word_search[first_mas_row_index][first_mas_col_index]);
        second_mas.push(word_search[second_mas_row_index][second_mas_col_index]);
    }

    println!("First: {}, Second: {}", first_mas, second_mas);

    if (first_mas == "MAS" || first_mas == "SAM") && (second_mas == "MAS" || second_mas == "SAM") {
        return 1;
    }

    //     let row_index = usize::try_from(position.0 + i as i64).expect("Invalid character!");
    //     let col_index = usize::try_from(position.1 + i as i64).expect("Invalid character!");

    //     possible_mas.push(word_search[row_index][col_index]);
    // }

    // if possible_mas == "MAS" || possible_mas == "SAM" {
    //     return 1;
    // }

    return 0;
}