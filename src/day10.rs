use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn run() {
    println!("--Part 1");
    run_part_01();
    println!("--Part 2");
    run_part_02();
}

fn run_part_01() {
    let file = File::open("input/day10.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut score = 0;

    for line in reader.lines() {
        let l = line.unwrap();

        let mut stack:Vec<char> = Vec::new();
        for c in l.chars() {
            match c {
                '(' | '[' | '{' | '<' => {
                    stack.push(c);
                },
                ')' | ']' | '}' | '>' => {
                    let expected = stack.pop();
                    match expected {
                        Option::None => {
                            score += score_for_missed(c);
                            break;
                        },
                        Option::Some(x) => {
                            match c {
                                ')' => if x != '(' { score += score_for_missed(c) },
                                ']' => if x != '[' { score += score_for_missed(c) },
                                '}' => if x != '{' { score += score_for_missed(c) },
                                '>' => if x != '<' { score += score_for_missed(c) },
                                _ => ()
                            }
                        }
                    }
                },
                _ => ()
            }
        }
    }

    println!("The score for corrupted lines is {}", score);
}

fn run_part_02() {
    let file = File::open("input/day10.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut scores:Vec<u64> = Vec::new();

    for line in reader.lines() {
        let l = line.unwrap();

        let mut stack:Vec<char> = Vec::new();

        let mut corrupted = false;
        for c in l.chars() {
            match c {
                '(' | '[' | '{' | '<' => {
                    stack.push(c);
                },
                ')' | ']' | '}' | '>' => {
                    let expected = stack.pop();
                    match expected {
                        Option::None => {
                            corrupted = true;
                            break;
                        },
                        Option::Some(x) => {
                            match c {
                                ')' => if x != '(' { 
                                    corrupted = true;
                                    break; 
                                },
                                ']' => if x != '[' { 
                                    corrupted = true;
                                    break;
                                 },
                                '}' => if x != '{' { 
                                    corrupted = true;
                                    break;
                                 },
                                '>' => if x != '<' { 
                                    corrupted = true;
                                    break;
                                 },
                                _ => ()
                            }
                        }
                    }
                },
                _ => ()
            }
        }

        if !corrupted {
            let mut local_score:u64 = 0;
            for c in stack.iter().rev() {
                local_score *= 5;
                local_score += score_for_inc(*c);
            }
            scores.push(local_score);
        }
    }

    scores.sort();

    let median_score = scores[scores.len() / 2];

    println!("The median score for autocomplete is {}", median_score);
}

fn score_for_missed(x:char) -> u32 {
    match x {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0
    }
}

fn score_for_inc(x:char) -> u64 {
    match x {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0
    }
}