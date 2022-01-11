use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::panic;

type Point = (i64, i64, i64);

enum Action {
    On, Off
}

struct Instruction {
    action: Action,
    min: Point,
    max: Point
}

struct Block {
    on: Volume,
    off: Vec<Volume>
}

impl Block {
    fn new(min: Point, max: Point) -> Self {
        Block{ on: Volume{min, max} , off: Vec::new() }
    }

    fn consider(&mut self, other: &Volume) {
        match self.on.intersect_volume(other) {
            Some(v) => {
                self.off.push(v);
            },
            None => ()
        }
    }

    fn count_on(&self) -> i64 {
        let mut volume:i64 = self.on.volume_of();

        let mut plus: Vec<Volume> = Vec::new();
        let mut minus: Vec<Volume> = Vec::new();
        for v in &self.off {
            volume -= v.volume_of();

            let mut to_add_minus:Vec<Volume> = Vec::new();
            for other in &plus {
                match v.intersect_volume(other) {
                    Some(intersection) => {
                        volume += intersection.volume_of();

                        to_add_minus.push(intersection);
                    },
                    None => ()
                }
            }

            for other in &minus {
                match v.intersect_volume(other) {
                    Some(intersection) => {
                        volume -= intersection.volume_of();

                        plus.push(intersection);
                    },
                    None => ()
                }
            }

            plus.push(v.clone());
            minus.append(&mut to_add_minus);
        }

        volume
    }
}

#[derive(Clone)]
struct Volume {
    min: Point,
    max: Point
}

impl Volume {
    fn volume_of(&self) -> i64 {
        (self.max.0 - self.min.0 + 1) * (self.max.1 - self.min.1 + 1) * (self.max.2 - self.min.2 + 1)
    }

    fn intersect_volume(&self, other: &Volume) -> Option<Volume> {
        let new_min = max(&self.min, &other.min);
        let new_max = min(&self.max, &other.max);

        if new_min.0 <= new_max.0 && new_min.1 <= new_max.1 && new_min.2 <= new_max.2 {
            Some(Volume { min:new_min, max:new_max })
        } else {
            None
        }
    }
}

pub fn run() {
    println!("--Part 1");
    run_part_01();
    println!("--Part 2");
    run_part_02();
}

fn run_part_01() {
    let file = File::open("input/day22test.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut map:HashSet<Point> = HashSet::new();

    let min = (-50, -50, -50);
    let max = (50, 50, 50);

    for line in reader.lines() {
        run_instruction(&mut map, parse(line.expect("Failed to read line").as_str()), &min, &max);
    }

    let num = map.len();

    println!("There are {} points on in the center", num);
}

fn run_part_02() {
    let file = File::open("input/day22.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut list:Vec<Block> = Vec::new();

    for line in reader.lines() {
        let ins = parse(line.expect("Failed to read line").as_str());

        match ins.action {
            Action::Off => {
                for block in list.iter_mut() {
                    block.consider(&Volume{ min: ins.min, max: ins.max});
                }
            },
            Action::On => {
                let new_block = Block::new(ins.min, ins.max);
                for block in list.iter_mut() {
                    block.consider(&new_block.on);
                }

                list.push(new_block);
            }
        }
    }

    let mut num = 0;
    for block in list {
        num += block.count_on();
    }


    println!("There are {} points on", num);
}

fn run_instruction(map: &mut HashSet<Point>, ins: Instruction, min: &Point, max: &Point) {
    let real_min_x = ins.min.0.max(min.0);
    let real_min_y = ins.min.1.max(min.1);
    let real_min_z = ins.min.2.max(min.2);
    let real_max_x = ins.max.0.min(max.0);
    let real_max_y = ins.max.1.min(max.1);
    let real_max_z = ins.max.2.min(max.2);
    for i in real_min_x..=real_max_x {
        for j in real_min_y..=real_max_y {
            for k in real_min_z..=real_max_z {

                match ins.action {
                    Action::On => {
                        map.insert((i, j, k));
                    },
                    Action::Off => {
                        map.remove(&(i, j, k));
                    }
                }
            }
        }
    }
}

fn parse(s: &str) -> Instruction {
    let ins_coord_pair:Vec<&str> = s.split_whitespace().collect();
    assert_eq!(ins_coord_pair.len(), 2);

    let coord_triplet = ins_coord_pair[1].split(",");    
    let mut coords:Vec<(i64, i64)> = Vec::new();
    for coord in coord_triplet {
        let reduced:Vec<&str> = coord.split("=").collect();
        assert_eq!(reduced.len(), 2);

        let hi_lo:Vec<&str> = reduced[1].split("..").collect();
        assert_eq!(hi_lo.len(), 2);
        coords.push((hi_lo[0].parse::<i64>().unwrap(), hi_lo[1].parse::<i64>().unwrap()));
    }
    assert_eq!(coords.len(), 3);

    let min = (coords[0].0.min(coords[0].1), coords[1].0.min(coords[1].1), coords[2].0.min(coords[2].1));
    let max = (coords[0].0.max(coords[0].1), coords[1].0.max(coords[1].1), coords[2].0.max(coords[2].1));

    let action = match ins_coord_pair[0] {
        "on" => Action::On,
        "off" => Action::Off,
        _ => panic!()
    };

    Instruction{ action, min, max}
}

fn min(p1: &Point, p2: &Point) -> Point {
    (p1.0.min(p2.0), p1.1.min(p2.1), p1.2.min(p2.2))
}

fn max(p1: &Point, p2: &Point) -> Point {
    (p1.0.max(p2.0), p1.1.max(p2.1), p1.2.max(p2.2))
}