#![feature(iter_array_chunks)]

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod timer;

use std::env;

fn main() {
    let arg: String = env::args()
        .nth(1)
        .expect("Expected number of day as argument.");

    let day: &str = arg.as_str();

    println!("Solving day {}:", day);

    match day {
        "1" => day_01::solve(),
        "2" => day_02::solve(),
        "3" => day_03::solve(),
        "4" => day_04::solve(),
        "5" => day_05::solve(),
        "6" => day_06::solve(),
        "7" => day_07::solve(),
        "8" => day_08::solve(),
        "9" => day_09::solve(),
        _ => println!("No solver for day {0}", day),
    }
}
