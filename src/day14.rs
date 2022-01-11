use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

pub fn run() {
    println!("--Part 1");
    run_part_01();
    println!("--Part 2");
    run_part_02();
}

fn run_part_01() {
    let file = File::open("input/day14.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    let mut source = lines.next().expect("Failed to parse first line").expect("Failed to parse first line");
    lines.next(); //Skip empty line

    let mut pair_ins_map:HashMap<String, String> = HashMap::new();

    for line in lines {
        let l = line.expect("Failed to parse line");

        let split = l.split(" -> ").collect::<Vec<&str>>();

        assert_eq!(split.len(), 2);

        pair_ins_map.insert(split[0].to_owned(), split[1].to_owned());
    }
    
    let iterations = 10 ;

    for _ in 0..iterations {
        let mut i = 2;

        while i <= source.len() {
            let last_pair = &source[i - 2..i];

            if let Some(x) = pair_ins_map.get(last_pair) {
                source.insert_str(i - 1, x.as_str());

                i += 1;
            }

            i += 1;
        }
    }

    let mut freq_map:HashMap<char, u64> = HashMap::new();

    for c in source.chars() {
        *freq_map.entry(c).or_insert(0) += 1;
    }

    let mut min_char = ' ';
    let mut min_num = u64::MAX;
    let mut max_char = ' ';
    let mut max_num = u64::MIN;

    for (key, value) in freq_map {
        if value < min_num {
            min_char = key;
            min_num = value;
        }

        if value > max_num {
            max_char = key;
            max_num = value;
        }
    }

    let difference = max_num - min_num;
    println!("The difference between the number of the most and least common element is {}", difference);
    println!("The most common element is {} and the least common is {}", max_char, min_char);
}

fn run_part_02() {
    let file = File::open("input/day14.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    let source = lines.next().expect("Failed to parse first line").expect("Failed to parse first line");
    lines.next(); //Skip empty line

    let mut pair_ins_map:HashMap<String, String> = HashMap::new();

    for line in lines {
        let l = line.expect("Failed to parse line");

        let split = l.split(" -> ").collect::<Vec<&str>>();

        assert_eq!(split.len(), 2);

        pair_ins_map.insert(split[0].to_owned(), split[1].to_owned());
    }

    let mut active_pairs:HashMap<String, u64> = HashMap::new();
    let mut freq_map:HashMap<char, u64> = HashMap::new();

    for i in 1..source.len() {
        *active_pairs.entry(source[(i - 1)..=i].to_owned()).or_insert(0) += 1;
    }

    source.chars().for_each(|c| *freq_map.entry(c).or_default() += 1);

    let iterations = 40;

    for _ in 0..iterations {
        let mut new_pairs:HashMap<String, u64> = HashMap::new();

        for (mut pair, num) in active_pairs.into_iter() {
            match pair_ins_map.get(&pair) {
                Some(ins) => {
                    *freq_map.entry(ins.chars().next().unwrap()).or_default() += num;
                    pair.insert_str(1, ins);
                    *new_pairs.entry(pair[0..=1].to_owned()).or_insert(0) += num;
                    *new_pairs.entry(pair[1..=2].to_owned()).or_insert(0) += num;
                },
                None => *new_pairs.entry(pair).or_insert(0) += num
            }
        }

        active_pairs = new_pairs;
    }

    let mut min_char = ' ';
    let mut min_num = u64::MAX;
    let mut max_char = ' ';
    let mut max_num = u64::MIN;

    for (key, value) in freq_map {
        if value < min_num {
            min_char = key;
            min_num = value;
        }

        if value > max_num {
            max_char = key;
            max_num = value;
        }
    }

    let difference = max_num - min_num;
    println!("The difference between the number of the most and least common element is {}", difference);
    println!("The most common element is {} and the least common is {}", max_char, min_char);
}