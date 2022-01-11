pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day25;

use std::env;
use std::time::Instant;

fn main() {
    let t = Instant::now();

    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        let day = args[1].clone();
        let num = day.as_str().parse::<i32>().expect("Failed to parse day number");
        get_fn(num)();
    } else {
        println!("Day Not Provided - Exiting");
    }

    let elapsed = t.elapsed();
    println!("Total Elapsed {:2?}", elapsed);
}

fn run_all() {
    for i in 1..=25 {
        println!("Running {}", i);

        let t = Instant::now();
        
        get_fn(i)();

        let elapsed = t.elapsed();
        println!("Elapsed {:2?}", elapsed);
        println!("----------------------------------------");
    }
}

fn get_fn(n: i32) -> fn() {
    match n {
        0 => run_all,
        1 => day01::run,
        2 => day02::run,
        3 => day03::run,
        4 => day04::run,
        5 => day05::run,
        6 => day06::run,
        7 => day07::run,
        8 => day08::run,
        9 => day09::run,
        10 => day10::run,
        11 => day11::run,
        12 => day12::run,
        13 => day13::run,
        14 => day14::run,
        15 => day15::run,
        16 => day16::run,
        17 => day17::run,
        18 => day18::run,
        19 => day19::run,
        20 => day20::run,
        21 => day21::run,
        22 => day22::run,
        23 => day23::run,
        25 => day25::run,
        _ => {
            println!("Day Not Found: {}", n);
            do_nothing
        }
    }
}

fn do_nothing() {

}