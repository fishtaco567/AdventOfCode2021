use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run() {
    let file = File::open("input/day01.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut last_num = i32::MAX;
    let last_nums:&mut [i32;4] = &mut[0, 0, 0, 0];

    let mut num_inc = 0;
    let mut num_inc_2 = 0;

    for (index, line) in reader.lines().enumerate() {
        let l = line.unwrap();
        let new_num = l.as_str().parse::<i32>().unwrap_or(i32::MIN);

        last_nums.rotate_left(1);
        last_nums[3] = new_num;

        if new_num > last_num {
            num_inc += 1;
        }
        last_num = new_num;

        if index >= (last_nums.len() - 1) {
            let s1:i32 = last_nums[0..3].iter().sum();
            let s2:i32 = last_nums[1..4].iter().sum();
            if s2 > s1 {
                num_inc_2 += 1;
            }
        }
    }

    println!("Depths Increased: {}", num_inc);
    println!("\"Averages\" Increased: {}", num_inc_2);
}