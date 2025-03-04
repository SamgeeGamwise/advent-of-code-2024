mod read_file;
mod equation2;
use time_elapsed::log_time_elapsed;

#[log_time_elapsed]
fn main() {
    let mut equations = read_file::read_file("input.txt");
    let mut total_calibration_result: f64 = 0.0;

    for i in 0..equations.len() {
        if equations[i].solve() {
            total_calibration_result += equations[i].answer;
        }
    }
    println!("{:?}", total_calibration_result);
}
