mod read_file;
mod day_four;
mod day_four_second;

fn main() {
    let word_search = read_file::file_to_2d_array("word_search.txt");
    println!("XMAS count: {}", day_four::count_all_xmas(word_search));
    println!("X-MAS count: {}", day_four_second::count_all_xmas(word_search));
}
