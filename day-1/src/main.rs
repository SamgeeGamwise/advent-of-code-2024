mod read_file;
mod day_one;

fn main() {
    let (values1, values2) = read_file::file_to_sorted_array("numbers.txt");

    let total_distance = day_one::find_difference_between_lists(values1, values2);
    let simularity_score = day_one::find_similarity_score(values1, values2);

    println!("Total difference: {}", total_distance);
    println!("Similarity Score: {}", simularity_score);
}
