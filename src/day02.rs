use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run() {

    println!("--Part 1");
    {
        let file = File::open("input/day02.txt").expect("Failed to open file");
        let reader = BufReader::new(file);
    
        let coords = simulate_part_01(reader);
    
        let (horiz, depth) = coords;
        let product = horiz * depth;
    
        println!("Horizontal Position is {}, Depth is {}", horiz, depth);
        println!("The product of the two is {}", product);
    }

    println!("--Part 2");
    {
        let file = File::open("input/day02.txt").expect("Failed to open file");
        let reader = BufReader::new(file);
        
        let coords = simulate_part_02(reader);

        let (horiz, depth) = coords;
        let product = horiz * depth;
    
        println!("CORRECTED Horizontal Position is {}, Depth is {}", horiz, depth);
        println!("CORRECTED Product of the two is {}", product);
    }
}

fn simulate_part_01(reader:BufReader<File>) -> (i32, i32) {
    let mut coords = (0, 0);

    for (_index, line) in reader.lines().enumerate() {
        let l = line.unwrap();
        let split:Vec<&str> = l.split_whitespace().collect();
        let num = split[1].parse::<i32>().unwrap();

        match split[0] {
            "forward" => {
                coords.0 += num;
            },
            "up" => {
                coords.1 -= num;
            },
            "down" => {
                coords.1 += num;
            },
            _ => {
                println!("Direction {} is not supported", split[0]);
            }
        }
    }

    coords
}

fn simulate_part_02(reader:BufReader<File>) -> (i32, i32) {
    let mut coords = (0, 0);
    let mut aim = 0;

    for (_index, line) in reader.lines().enumerate() {
        let l = line.unwrap();
        let split:Vec<&str> = l.split_whitespace().collect();
        let num = split[1].parse::<i32>().unwrap();

        match split[0] {
            "forward" => {
                coords.0 += num;
                coords.1 += aim * num;
            },
            "up" => {
                aim -= num;
            },
            "down" => {
                aim += num;
            },
            _ => {
                println!("Direction {} is not supported", split[0]);
            }
        }
    }

    coords
}