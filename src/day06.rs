use std::fs;

pub fn run() {
    println!("After 80 days, there are {} fish", calculate_fish_for(80));
    println!("After 256 days, there are {} fish", calculate_fish_for(256));
}

fn calculate_fish_for(days:u32) -> u64 {
   let input = fs::read_to_string("input/day06.txt").expect("Failed to read file"); 

   let input_numbers = input.split(',').map(|x| x.parse::<usize>().expect("Failed to parse number")).collect::<Vec<usize>>();

   let mut fish_numbers:[u64; 9] = [0; 9];

   for num in input_numbers {
       fish_numbers[num] += 1;
   }

   for _ in 0..days {
       fish_numbers.rotate_left(1);
       fish_numbers[6] += fish_numbers[8].clone();
   }

   fish_numbers.iter().sum::<u64>()
}