mod read_file;
mod day_two;

fn main() {
    let mut safe_count: u32 = 0;
    let reports: [day_two::Report; 1000] = read_file::file_to_records("numbers.txt");

    for mut report in reports {
        if report.is_safe() {
            safe_count += 1;
        } else {
            if true {
                let mut report_copy = report.clone();

                if report_copy.dampen(0).is_safe() || report.dampen(1).is_safe() {
                    println!("Dampening safe!");
                    safe_count += 1;
                }
            } else if report.dampen(1).is_safe() {
                safe_count += 1;
            }
        }
    }

    println!("Safe count: {}", safe_count);
}
