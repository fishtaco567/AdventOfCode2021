use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

pub fn run() {
    println!("--Part 1");
    run_part_01();
    println!("--Part 2");
    run_part_02();
}

fn run_part_01() {
    let file = File::open("input/day13.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut map:HashSet<(i32, i32)> = HashSet::new();

    let mut lines = reader.lines();

    //Iterate coord
    while let Some(Ok(s)) = lines.next() {
        if s.is_empty() {
            break;
        }

        let numbers = s.split(",").map(|x| x.parse::<i32>().expect("Could not parse dot coordinate")).collect::<Vec<i32>>();
        
        assert_eq!(numbers.len(), 2);

        map.insert((numbers[0], numbers[1]));
    }

    //Iterate folds
    while let Some(Ok(s)) = lines.next() {
        let inst = &s[11..];
        let split = inst.split("=").collect::<Vec<&str>>();
        
        assert_eq!(split.len(), 2);

        let num = split[1].parse::<i32>().expect("Cannot parse fold coordinate");

        let mut new_map:HashSet<(i32, i32)> = HashSet::new();

        match split[0] {
            "x" => {
                for point in &map {
                    if point.0 > num {
                        new_map.insert((2 * num - point.0, point.1));
                    } else {
                        new_map.insert((point.0, point.1));
                    }
                }
            },
            "y" => {
                for point in &map {
                    if point.1 > num {
                        new_map.insert((point.0, 2 * num - point.1));
                    } else {
                        new_map.insert((point.0, point.1));
                    }
                }
            },
            _ => ()
        }

        map = new_map;

        break;
    }

    println!("There are {} points after one fold", map.len());
}

fn run_part_02() {
    let file = File::open("input/day13.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut map:HashSet<(i32, i32)> = HashSet::new();

    let mut lines = reader.lines();

    //Iterate coord
    while let Some(Ok(s)) = lines.next() {
        if s.is_empty() {
            break;
        }

        let numbers = s.split(",").map(|x| x.parse::<i32>().expect("Could not parse dot coordinate")).collect::<Vec<i32>>();
        
        assert_eq!(numbers.len(), 2);

        map.insert((numbers[0], numbers[1]));
    }

    //Iterate folds
    while let Some(Ok(s)) = lines.next() {
        let inst = &s[11..];
        let split = inst.split("=").collect::<Vec<&str>>();
        
        assert_eq!(split.len(), 2);

        let num = split[1].parse::<i32>().expect("Cannot parse fold coordinate");

        let mut new_map:HashSet<(i32, i32)> = HashSet::new();

        match split[0] {
            "x" => {
                for point in &map {
                    if point.0 > num {
                        new_map.insert((2 * num - point.0, point.1));
                    } else {
                        new_map.insert((point.0, point.1));
                    }
                }
            },
            "y" => {
                for point in &map {
                    if point.1 > num {
                        new_map.insert((point.0, 2 * num - point.1));
                    } else {
                        new_map.insert((point.0, point.1));
                    }
                }
            },
            _ => ()
        }

        map = new_map;
    }

    println!("There are {} points after all folds", map.len());

    let max_x = map.iter().fold(0, |acc,x| if x.0 > acc { x.0 } else { acc });
    let max_y = map.iter().fold(0, |acc,x| if x.1 > acc { x.1 } else { acc });

    for y in 0..=max_y {
        for x in 0..=max_x {
            if map.contains(&(x, y)) {
                print!("xx");
            } else {
                print!("  ");
            }
        } 

        println!("");
    }
}