mod timer;
mod day_01;

use std::env;

fn main() {
    let arg: String = env::args()
        .nth(1)
        .expect("Expected number of day as argument.");

    let day: &str = arg.as_str();

    match day {
        "1" => day_01::solve(),
        _ => println!("No solver for day {0}", day),
    }
}
