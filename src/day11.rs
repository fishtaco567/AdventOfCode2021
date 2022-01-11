use std::fs::File;
use std::io::{BufRead, BufReader};

struct JellyMap {
    jelly_phase: Vec<u32>,
    width:usize,
    height:usize,
}

impl JellyMap {
    fn new(lines: Vec<String>) -> Self {
        let mut map = JellyMap {
            jelly_phase: Vec::new(),
            width: lines[0].len(),
            height: lines.len(),
        };

        for line in lines {
            for char in line.chars() {
                map.jelly_phase.push(char.to_digit(10).expect("Could not parse digit"));
            }
        }

        map
    }

    fn step(&mut self) -> u32 {
        let mut flash:Vec<u32> = vec![0; self.jelly_phase.len()];

        self.inc_all();

        let mut queue:Vec<(usize, usize)> = Vec::new();

        for (i, x) in self.jelly_phase.iter().enumerate() {
            if *x > 9 {
                queue.push((i % self.width, i / self.width));
                flash[i] = 1;
            }
        }

        while !queue.is_empty() {
            let next = queue.pop().unwrap();
            let mut to_add = self.inc_adj(next, &mut flash);
            queue.append(&mut to_add);
        }

        for i in self.jelly_phase.iter_mut() {
            if *i > 9 {
                *i = 0;
            }
        }

        flash.iter().sum()
    }

    fn inc_adj(&mut self, x: (usize, usize), flash:&mut Vec<u32>) -> Vec<(usize, usize)> {
        let mut over_nine = Vec::new();

        let x_min = if x.0 != 0 { x.0 - 1 } else { x.0 };
        let y_min = if x.1 != 0 { x.1 - 1 } else { x.1 };
        let x_max = if x.0 != self.width - 1 { x.0 + 1 } else { x.0 };
        let y_max = if x.1 != self.height - 1 { x.1 + 1 } else { x.1 };

        for j in y_min..=y_max {
            for i in x_min..=x_max {
                if i == x.0 && j == x.1 {
                    continue;
                }

                let index = i + j * self.width;
                self.jelly_phase[index] += 1;
                if self.jelly_phase[index] > 9 && flash[index] == 0 {
                    flash[index] = 1;
                    over_nine.push((i, j));
                }
            }
        }

        over_nine
    }

    fn inc_all(&mut self) {
        self.jelly_phase.iter_mut().for_each(|x| *x += 1);
    }
}

pub fn run() {
    run_part_01();
    run_part_02();
}

fn run_part_01() {
    let file = File::open("input/day11.txt").expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|x| x.expect("Could not read line")).collect::<Vec<String>>();

    let mut jelly_map = JellyMap::new(lines);

    let mut flashes = 0;

    for _ in 0..100 {
        flashes += jelly_map.step();
    }

    println!("There were {} flashes in 100 steps", flashes);
}

fn run_part_02() {
    let file = File::open("input/day11.txt").expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|x| x.expect("Could not read line")).collect::<Vec<String>>();

    let mut jelly_map = JellyMap::new(lines);

    let mut step = 0;
    while jelly_map.step() != jelly_map.jelly_phase.len() as u32 {
        step += 1;
    }

    println!("The first synchronization was at {} steps", step + 1);
}