pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        let day = args[1].clone();
        let num = day.as_str().parse::<i32>().unwrap();
        match num {
            1 => day01::run(),
            2 => day02::run(),
            3 => day03::run(),
            4 => day04::run(),
            5 => day05::run(),
            _ => println!("Day Not Found: {}", num),
        }
    } else {
        println!("Day Not Provided - Exiting");
    }
}