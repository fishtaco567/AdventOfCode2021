use std::fs;

pub fn run() {
    println!("--Part 1");
    run_part_01();
    println!("--Part 2");
    run_part_02();
}

fn run_part_01() {
    let input = fs::read_to_string("input/day07.txt").expect("Failed to read file"); 
 
    let input_numbers = input.split(',').map(|x| x.parse::<i32>().expect("Failed to parse number")).collect::<Vec<i32>>();

    let mut lowest_fuel = i32::MAX;
    
    let mut current_goal:i32 = 0;

    while current_goal < 10000 {
        let new_fuel = input_numbers.iter().fold(0, |acc, x| acc + (*x - current_goal).abs());
        if new_fuel > lowest_fuel {
            current_goal -= 1;
            break;
        }
        lowest_fuel = new_fuel;

        current_goal += 1;
    }

    println!("At position {}, the crabs will use {} fuel, the lowest", current_goal, lowest_fuel);
}

fn run_part_02() {
    let input = fs::read_to_string("input/day07.txt").expect("Failed to read file"); 
 
    let input_numbers = input.split(',').map(|x| x.parse::<i32>().expect("Failed to parse number")).collect::<Vec<i32>>();

    let mut lowest_fuel = i32::MAX;
    
    let mut current_goal:i32 = 0;

    while current_goal < 10000 {
        let new_fuel = input_numbers.iter().fold(0, |acc, x| acc + tri_num((*x - current_goal).abs()));

        if new_fuel > lowest_fuel {
            current_goal -= 1;
            break;
        }
        lowest_fuel = new_fuel;

        current_goal += 1;
    }

    println!("At position {}, the crabs will actually use {} fuel, the lowest", current_goal, lowest_fuel);
}

fn tri_num(n:i32) -> i32 {
    (n * (n + 1)) / 2
}