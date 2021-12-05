use core::num;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};

struct SubDiagnostic {
    current_diag_code: Vec<i32>,
    total_codes: i32,
}

impl SubDiagnostic {
    fn add_code(&mut self, s: &str) {
        let chars: Vec<char> = s.chars().collect();

        for i in 0..s.len() {

            if chars[i] == '1' {
                self.current_diag_code[i] += 1;
            }
        }

        self.total_codes += 1;
    }

    fn new(size: usize) -> Self {
        SubDiagnostic {
            current_diag_code: vec![0; size],
            total_codes: 0,
        }
    }

    fn retrieve_codes(&self) -> (String, String) {
        let threshhold = self.total_codes / 2;
        let gamma_rate = self.current_diag_code.iter().map(|x| if x >= &threshhold { '1' } else { '0' }).collect::<String>();
        let epsilon_rate = self.current_diag_code.iter().map(|x| if x >= &threshhold { '0' } else { '1' }).collect::<String>();
        (gamma_rate, epsilon_rate)
    }
}

pub fn run() {
    run_part_01();
    run_part_02();
}

fn run_part_01() {
    let file = File::open("input/day03.txt").expect("Failed to open file");
    let mut reader = BufReader::new(file);
    
    let mut first_line = String::new();
    let _ = reader.read_line(&mut first_line);

    first_line = first_line.split_whitespace().collect();

    let _ = reader.seek(SeekFrom::Start(0));

    let diag_size = first_line.len();

    let mut sub_diag = SubDiagnostic::new(diag_size);

    for (_index, line) in reader.lines().enumerate() {
        sub_diag.add_code(line.unwrap().as_str());
    }

    let code_string = sub_diag.retrieve_codes();
    let gamma_rate = i32::from_str_radix(code_string.0.as_str(), 2).expect("Cannot parse resulting binary for gamma rate");
    let epsilon_rate = i32::from_str_radix(code_string.1.as_str(), 2).expect("Cannot parse resulting binary for epsilon rate");

    let power_consumption = gamma_rate * epsilon_rate;

    println!("The power consumption is {}", power_consumption);
}

fn run_part_02() {
    let file = File::open("input/day03.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut lines_oxy = reader.lines().map(|line| 
        line.expect("Cound not read line").split_whitespace().collect::<String>().chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    
    let mut lines_co2 = lines_oxy.clone();
        
    let max_place = lines_oxy[0].len();
    let mut place = 0;

    while lines_oxy.len() > 1 && place < max_place {
        let num_occ = lines_oxy.iter().fold(0, |acc, line| if line[place] == '1' { acc + 1 } else { acc });
        let num_not = lines_oxy.len() - num_occ;
        let good_digit = if num_occ >= num_not { '1' } else { '0' };
        lines_oxy = lines_oxy.iter().filter(|line| line[place] == good_digit).map(|line| line.clone()).collect();
        place += 1;
    }

    place = 0;

    while lines_co2.len() > 1 && place < max_place {
        let num_occ = lines_co2.iter().fold(0, |acc, line| if line[place] == '1' { acc + 1 } else { acc });
        let num_not = lines_co2.len() - num_occ;
        let good_digit = if num_not <= num_occ { '0' } else { '1' };
        lines_co2 = lines_co2.iter().filter(|line| line[place] == good_digit).map(|line| line.clone()).collect();
        place += 1;
    }

    let oxy_rating_str = lines_oxy[0].iter().collect::<String>();
    let co2_rating_str = lines_co2[0].iter().collect::<String>();
    
    let oxy_rating = i32::from_str_radix(oxy_rating_str.as_str(), 2).expect("Could not parse oxygen rating");
    let co2_rating = i32::from_str_radix(co2_rating_str.as_str(), 2).expect("Cound not parse co2 rating");

    let life_support_rating = oxy_rating * co2_rating;
    println!("The life support rating is {}", life_support_rating);
}