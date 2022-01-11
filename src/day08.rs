use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::io::{BufReader, BufRead};

struct Digit {
    num: Option<u32>,
    characters: HashSet<char>,
}

impl Digit {
    fn new(seq: &str) -> Self {
        let mut digit = Digit {
            num: match seq.len() {
                2 => Option::Some(1),
                3 => Option::Some(7),
                4 => Option::Some(4),
                7 => Option::Some(8),
                _ => Option::None,
            },
            characters: HashSet::new(),
        };

        for c in seq.chars() {
            digit.characters.insert(c);
        }
        
        digit
    }

    fn contains(&self, other: &Digit) -> bool {
        self.characters.is_superset(&other.characters)
    }

    fn contains_some(&self, other: &Digit) -> u32 {
        other.characters.iter().fold(0, |acc, x| if self.characters.contains(x) { acc + 1 } else { acc })
    }

    fn get_number_of_chars(&self) -> usize {
        self.characters.len()
    }

    fn assign_num(&mut self, known: &Vec<&Digit>) {
        for d in known {
            if self.characters == d.characters {
                self.num = d.num;
                break;
            }
        }
    }

    fn set_num(&mut self, num: u32) {
        self.num = Some(num);
    }
}

pub fn run() {
    println!("--Part 1");
    run_part_01();
    println!("--Part 2");
    run_part_02();
}

fn run_part_01() {
    let file = File::open("input/day08.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut num_chars: u32 = 0;

    for line in reader.lines() {
        let l = line.expect("Failed to read line");
        let split = l.split('|').collect::<Vec<&str>>()[1].split_whitespace().collect::<Vec<&str>>();
        for digit in split.iter() {
            let num_char = digit.len();
            match num_char {
                2 => num_chars += 1,
                4 => num_chars += 1,
                3 => num_chars += 1,
                7 => num_chars += 1,
                _ => (),
            }
        }
    }

    println!("There are {} 1s, 4s, 7s, or 8s", num_chars);
}

fn run_part_02() {
    let file = File::open("input/day08.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut sum = 0;

    for line in reader.lines() {
        let mut unknown_fives:Vec<Digit> = Vec::new();
        let mut unknown_sixes:Vec<Digit> = Vec::new();

        let mut digit_map:HashMap<u32, Digit> = HashMap::new();

        let l = line.expect("Failed to read line");
        
        let split = l.split('|').collect::<Vec<&str>>();

        let first_part = split[0].split_whitespace().collect::<Vec<&str>>();
        let second_part = split[1].split_whitespace().collect::<Vec<&str>>();
        for digit_str in first_part.iter() {
            let new_digit = Digit::new(digit_str.clone());
            match new_digit.num {
                None => {
                    match new_digit.get_number_of_chars() {
                        5 => unknown_fives.push(new_digit),
                        6 => unknown_sixes.push(new_digit),
                        _ => (),
                    }
                },
                Some(x) => {
                    digit_map.insert(x, new_digit);
                    ()
                },
            }
        }

        for mut digit in unknown_fives.into_iter() {
            if digit.contains(digit_map.get(&1).expect("Could not find one")) {
                digit.set_num(3);
            } else if digit.contains_some(digit_map.get(&4).expect("Could not find four")) == 2 {
                digit.set_num(2);
            } else {
                digit.set_num(5);
            }
            digit_map.insert(digit.num.unwrap(), digit);
        }

        for mut digit in unknown_sixes.into_iter() {
            if digit.contains(digit_map.get(&4).expect("Could not find four")) {
                digit.set_num(9);
            } else if digit.contains(digit_map.get(&1).expect("Could not find one")) {
                digit.set_num(0);
            } else {
                digit.set_num(6);
            }
            digit_map.insert(digit.num.unwrap(), digit);
        }

        let digit_list:Vec<&Digit> = digit_map.values().collect();

        let mut output_num = 0;
        let mut place = 1000;

        for digit_str in second_part.iter() {            
            let mut new_digit = Digit::new(digit_str.clone());
            new_digit.assign_num(&digit_list);
            output_num += new_digit.num.unwrap() * place;
            place /= 10;
        }

        sum += output_num;
    }

    println!("The sum of all outputs is {}", sum);
}