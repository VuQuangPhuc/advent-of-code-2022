#![feature(iter_array_chunks)]

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod timer;

use std::env;

fn main() {
    let arg: String = env::args()
        .nth(1)
        .expect("Expected number of day as argument.");

    let day: &str = arg.as_str();

    match day {
        "1" => day_01::solve(),
        "2" => day_02::solve(),
        "3" => day_03::solve(),
        "4" => day_04::solve(),
        "5" => day_05::solve(),
        _ => println!("No solver for day {0}", day),
    }
}
