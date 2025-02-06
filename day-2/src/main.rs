mod read_file;
mod day_two;

fn main() {
    let mut safe_count: u32 = 0;
    let reports: [day_two::Report; 1000] = read_file::file_to_records("numbers.txt");

    for mut report in reports {
        if report.is_safe() || report.dampened_safe() {
            safe_count += 1;
        }
    }

    println!("Safe count: {}", safe_count);
}
