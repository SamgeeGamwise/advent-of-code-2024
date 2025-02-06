mod read_file;
mod day_three;

fn main() {
    let memory: String = read_file::file_to_records("memory.txt");
    let multiplations: Vec<(u32, u32)> = day_three::scroll_string_for_mul(memory);
    let mut multiplication_sum: u128 = 0;

    for multiplication in multiplations {
        multiplication_sum += u128::from(multiplication.0 * multiplication.1);
    }

    println!("multiplication_sum: {}", multiplication_sum);
}
