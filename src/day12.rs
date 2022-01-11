use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

#[derive(Clone)]
struct Path<'a> {
    visited_small_num:HashMap<&'a str, i32>,
    visited:Vec<&'a str>,
    head:&'a str,
    used_extra:bool,
}

impl<'a> Path<'a> {
    fn new() -> Self {
        let mut p = Path{
            visited_small_num: HashMap::new(),
            visited: Vec::new(),
            head: "start",
            used_extra: false,
        };

        p.visited.push("start");
        p.visited_small_num.insert("start", i32::MAX);

        p
    }

    fn child_to(&self, new_node: &'a str, extra:bool) -> Option<Self> {
        let mut used_extra = self.used_extra;
        
        if new_node == "start" {
            return None;
        }

        if is_lowercase(new_node) {
            match self.visited_small_num.get(new_node) {
                Some(x) => { 
                    if *x >= 1 { 
                        if extra && !used_extra {
                            used_extra = true;
                        } else {
                            return None 
                        }
                    } 
                },
                None => ()
            }
        }

        let mut x = self.clone();

        *x.visited_small_num.entry(new_node).or_insert(0) += 1;
        x.visited.push(new_node);
        x.head = new_node;
        x.used_extra = used_extra;

        Some(x)
    }

    fn create_path_string(&self) -> String {
        let mut path_string = String::new();
        for node in &self.visited {
            path_string.push_str(node);
            path_string.push_str(",");
        }

        path_string
    }
}

pub fn run() {
    run_part_01();
    run_part_02();
}

fn run_part_01() {
    let file = File::open("input/day12.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut map:HashMap<String, Vec<String>> = HashMap::new();

    for line in reader.lines() {
        let l = line.expect("Failed to parse line");
        let split = l.split("-").collect::<Vec<&str>>();
    
        assert_eq!(split.len(), 2);
    
        map.entry(split[0].to_owned()).or_insert(Vec::new()).push(split[1].to_owned());
        map.entry(split[1].to_owned()).or_insert(Vec::new()).push(split[0].to_owned());
    }

    let mut current_paths:Vec<Path> = Vec::new();

    current_paths.push(Path::new());

    let mut found_paths:Vec<String> = Vec::new();

    while let Some(x) = current_paths.pop() {
        if let Some(connections) = map.get(x.head) {
            for node in connections {
                if let Some(new_path) = x.child_to(node.as_str(), false) {
                    if new_path.head == "end" {
                        found_paths.push(x.create_path_string());
                    } else {
                        current_paths.push(new_path);
                    }
                }
            }
        }
    }

    println!("There were {} paths", found_paths.len());
}

fn run_part_02() {
    let file = File::open("input/day12.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut map:HashMap<String, Vec<String>> = HashMap::new();

    for line in reader.lines() {
        let l = line.expect("Failed to parse line");
        let split = l.split("-").collect::<Vec<&str>>();
    
        assert_eq!(split.len(), 2);
    
        map.entry(split[0].to_owned()).or_insert(Vec::new()).push(split[1].to_owned());
        map.entry(split[1].to_owned()).or_insert(Vec::new()).push(split[0].to_owned());
    }

    let mut current_paths:Vec<Path> = Vec::new();

    current_paths.push(Path::new());

    let mut found_paths:Vec<String> = Vec::new();

    while let Some(x) = current_paths.pop() {
        if let Some(connections) = map.get(x.head) {
            for node in connections {
                if let Some(new_path) = x.child_to(node.as_str(), true) {
                    if new_path.head == "end" {
                        found_paths.push(x.create_path_string());
                    } else {
                        current_paths.push(new_path);
                    }
                }
            }
        }
    }

    println!("There were {} paths for the two-visit solution", found_paths.len());
}

fn is_lowercase(s: &str) -> bool {
    if s.to_lowercase() == s {
        true
    } else {
        false
    }
}