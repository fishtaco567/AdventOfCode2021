use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, BufRead};

struct VentHeightmap {
    heightmap: Vec<u32>,
    width: usize,
    height: usize,
}

impl VentHeightmap {
    fn new(lines: Vec<String>) -> Self {
        let w = lines[0].len();
        let h = lines.len();
        let mut vh = VentHeightmap {
            heightmap: Vec::with_capacity(w * h),
            width: w,
            height: h,
        };

        for line in lines {
            for c in line.chars() {
                vh.heightmap.push(c.to_digit(10).expect("Could not parse digit"));
            }
        }

        vh
    }

    fn get_height(&self, x:usize, y:usize) -> Option<&u32> {
        if x > self.width || y > self.height {
            Option::None
        } else {
            self.heightmap.get(x + y * self.width)
        }
    }

    fn is_lowest(&self, x:usize, y:usize) -> Option<bool> {
        if x > self.width || y > self.height {
            Option::None
        } else {
            let center_cell = self.get_height(x, y).unwrap();
            
            if x != 0 && self.get_height(x - 1, y).unwrap() <= center_cell {
                return Option::Some(false)
            }

            if y != 0 && self.get_height(x, y - 1).unwrap() <= center_cell {
                return Option::Some(false)
            }

            if x != self.width - 1 && self.get_height(x + 1, y).unwrap() <= center_cell {
                return Option::Some(false)
            }
            
            if y != self.height - 1 && self.get_height(x, y + 1).unwrap() <= center_cell {
                return Option::Some(false)
            }

            Option::Some(true)
        }
    }

    fn basin_size(&self, x:usize, y:usize) -> Option<u32> {
        if x > self.width || y > self.height {
            return Option::None
        }
        
        let mut inside:HashSet<(usize, usize)> = HashSet::new();
        let mut q:Vec<(usize, usize)> = Vec::new();
        q.push((x, y));

        while !q.is_empty() {
            let n = q.swap_remove(0);
            if !inside.contains(&n) && *self.get_height(n.0, n.1).unwrap() != 9 {
                inside.insert(n);
                if n.0 > 0 {
                    q.push((n.0 - 1, n.1));
                }
                if n.0 < self.width - 1 {
                    q.push((n.0 + 1, n.1));
                }
                if n.1 > 0 {
                    q.push((n.0, n.1 - 1));
                }
                if n.1 < self.height - 1 {
                    q.push((n.0, n.1 + 1));
                }
            }
        }

        Option::Some(inside.len() as u32)
    }

    fn _show(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.get_height(x, y).unwrap());
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
    let file = File::open("input/day09.txt").expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|x| x.expect("Could not read line")).collect::<Vec<String>>();

    let vent_map = VentHeightmap::new(lines);

    let mut total_risk = 0;

    for y in 0..vent_map.height {
        for x in 0..vent_map.width {
            if vent_map.is_lowest(x, y).unwrap() {
                total_risk += vent_map.get_height(x, y).unwrap() + 1;
            }
        }
    }

    println!("The total risk level is {}", total_risk);
}

fn run_part_02() {
    let file = File::open("input/day09.txt").expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|x| x.expect("Could not read line")).collect::<Vec<String>>();

    let vent_map = VentHeightmap::new(lines);

    let mut three_largest = vec![0, 0, 0];

    for y in 0..vent_map.height {
        for x in 0..vent_map.width {
            if vent_map.is_lowest(x, y).unwrap() {
                let basin_size = vent_map.basin_size(x, y).unwrap();

                if basin_size > three_largest[0] {
                    three_largest[0] = basin_size;
                    three_largest.sort();
                }
            }
        }
    }

    println!("The product of the three largest basins is {}", three_largest.iter().product::<u32>());
}