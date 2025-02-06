mod read_file;
mod day_two;

fn main() {
    let mut safe_count: u32 = 0;
    let reports: [day_two::Report; 1000] = read_file::file_to_records("numbers.txt");

    for mut report in reports {
        let (safe, index) = report.is_safe();

        if safe {
            safe_count += 1;
        } else {
            if index == 1 {
                let mut report_copy = report.clone();

                if report_copy.dampen(0).is_safe().0 || report.dampen(1).is_safe().0 {
                    println!("Dampening safe!");
                    safe_count += 1;
                }
            } else if report.dampen(index).is_safe().0 {
                safe_count += 1;
            }
        }
    }

    println!("Safe count: {}", safe_count);
}
