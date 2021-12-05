use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use std::cmp;

type Point = (u32, u32);

struct VentMap {
    vents: HashMap<Point, u32>,
}

impl VentMap {
    fn new() -> Self {
        VentMap {
            vents: HashMap::new(),
        }
    }

    fn add_line_ortho(&mut self, line: &str) {
        let coord_pairs = line.split_whitespace().collect::<Vec<&str>>();
        let pair1 = get_coords(coord_pairs[0]);
        let pair2 = get_coords(coord_pairs[2]);

        if pair1[0] == pair2[0] {
            let x = pair1[0];

            let lower = cmp::min(pair1[1], pair2[1]);
            let upper = cmp::max(pair1[1], pair2[1]);
            for y in lower..=upper {
                let point: Point = (x, y);
                *self.vents.entry(point).or_insert(0) += 1;
            }
        } else if pair1[1] == pair2[1] {
            let y = pair1[1];

            let lower = cmp::min(pair1[0], pair2[0]);
            let upper = cmp::max(pair1[0], pair2[0]);
            for x in lower..=upper {
                let point: Point = (x, y);
                *self.vents.entry(point).or_insert(0) += 1;
            }
        }
    }

    fn add_line(&mut self, line: &str) {
        let coord_pairs = line.split_whitespace().collect::<Vec<&str>>();
        let pair1 = get_coords(coord_pairs[0]);
        let pair2 = get_coords(coord_pairs[2]);

        if pair1[0] == pair2[0] {
            let x = pair1[0];

            let lower = cmp::min(pair1[1], pair2[1]);
            let upper = cmp::max(pair1[1], pair2[1]);
            for y in lower..=upper {
                let point: Point = (x, y);
                *self.vents.entry(point).or_insert(0) += 1;
            }
        } else if pair1[1] == pair2[1] {
            let y = pair1[1];

            let lower = cmp::min(pair1[0], pair2[0]);
            let upper = cmp::max(pair1[0], pair2[0]);
            for x in lower..=upper {
                let point: Point = (x, y);
                *self.vents.entry(point).or_insert(0) += 1;
            }
        } else {
            if pair1[0] < pair2[0] && pair1[1] < pair2[1] {
                for p in (pair1[0]..=pair2[0]).zip(pair1[1]..=pair2[1]) {
                    *self.vents.entry(p).or_insert(0) += 1;
                }
            } else if pair1[0] < pair2[0] && pair1[1] > pair2[1] {
                for p in (pair1[0]..=pair2[0]).zip((pair2[1]..=pair1[1]).rev()) {
                    *self.vents.entry(p).or_insert(0) += 1;
                }
            } else if pair1[0] > pair2[0] && pair1[1] > pair2[1] {
                for p in (pair2[0]..=pair1[0]).rev().zip((pair2[1]..=pair1[1]).rev()) {
                    *self.vents.entry(p).or_insert(0) += 1;
                }
            } else if pair1[0] > pair2[0] && pair1[1] < pair2[1] {
                for p in (pair2[0]..=pair1[0]).rev().zip(pair1[1]..=pair2[1]) {
                    *self.vents.entry(p).or_insert(0) += 1;
                }
            }
        }
    }

    fn get_places_in_excess_of(&self, x: u32) -> u32 {
        self.vents.values().fold(0, |acc, num| if num >= &x { acc + 1 } else { acc })
    }

    fn display(&self, x:u32, y:u32) {
        for j in 0..y {
            for i in 0..x {
                let point:Point = (i, j);
                let num = self.vents.get(&point);
                match num {
                    Some(x) => print!("{}", x),
                    None => print!("."),
                }
            }
            println!("");
        }
    }
}

pub fn run() {
    run_part_01();
    run_part_02();
}

fn run_part_01() {
    let file = File::open("input/day05.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut vents  = VentMap::new();

    for line in reader.lines() {
        vents.add_line_ortho(line.unwrap().as_str());
    }

    let num_over_two = vents.get_places_in_excess_of(2);
    println!("{}", num_over_two);
}

fn run_part_02() {
    let file = File::open("input/day05.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut vents  = VentMap::new();

    for line in reader.lines() {
        vents.add_line(line.unwrap().as_str());
    }

    let num_over_two = vents.get_places_in_excess_of(2);
    println!("{}", num_over_two);
}

fn get_coords(s: &str) -> Vec<u32> {
    s.split(',').map(|x| x.parse::<u32>().expect("Failed to parse coord")).collect::<Vec<u32>>()
}