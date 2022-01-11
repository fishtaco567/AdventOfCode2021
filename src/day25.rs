use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Clone)]
enum SeaCucumber {
    Down,
    Right,
    None
}

#[derive(Clone)]
struct Map {
    map: Vec<SeaCucumber>,
    width: usize,
    height: usize
}

impl Map {
    fn new_with_size(width:usize, height:usize) -> Self {
        let len = width * height;

        Map { map: (0..len).map(|_| SeaCucumber::None).collect(), width, height }
    }

    fn new_from_lines(lines: Vec<String>) -> Self {
        let width = lines[0].len();
        let height = lines.len();
        
        println!("Size is {} {}", width, height);

        let len = width * height;
        let mut map = Vec::with_capacity(len);

        for line in lines {
            for char in line.chars() {
                let sc = match char {
                    '.' => SeaCucumber::None,
                    'v' => SeaCucumber::Down,
                    '>' => SeaCucumber::Right,
                    _ => SeaCucumber::None
                };

                map.push(sc);
            }
        }

        Map { map, width, height }
    }

    fn get_right_coords(&self, x:usize, y:usize) -> (usize, usize) {
        ((x + 1) % self.width, y)
    }

    fn get_down_coords(&self, x:usize, y:usize) -> (usize, usize) {
        (x, (y + 1) % self.height)
    }
    
    fn get_at(&self, x:usize, y:usize) -> Option<&SeaCucumber> {
        self.map.get(self.index_of(x, y))
    }

    fn set_at(&mut self, x:usize, y:usize, sea_cuc:SeaCucumber) -> bool {
        let i = self.index_of(x, y);
        if i < self.map.len() {
            self.map[i] = sea_cuc;
            true
        } else {
            false
        }
    }

    fn index_of(&self, x:usize, y:usize) -> usize {
        x + y * self.width
    }

    fn step(&self) -> (Self, bool) {
        let mut next = Map::new_with_size(self.width, self.height);

        let mut moved = false;

        //Step right
        for y in 0..self.height {
            for x in 0..self.width {
                match self.get_at(x, y) {
                    Some(SeaCucumber::Right) => {
                        let (rx, ry) = self.get_right_coords(x, y);
                        match self.get_at(rx, ry) {
                            Some(SeaCucumber::None) => {
                                next.set_at(rx, ry, SeaCucumber::Right);
                                moved = true;
                            },
                            _ => {
                                next.set_at(x, y, SeaCucumber::Right);
                            }
                        }
                    },
                    Some(SeaCucumber::Down) => {
                        next.set_at(x, y, SeaCucumber::Down);
                    },
                    _ => ()
                }
            }
        }

        let int = next.clone();
        next = Map::new_with_size(int.width, int.height);

        //Step down
        for y in 0..int.height {
            for x in 0..int.width {
                match int.get_at(x, y) {
                    Some(SeaCucumber::Down) => {
                        let (rx, ry) = int.get_down_coords(x, y);
                        match int.get_at(rx, ry) {
                            Some(SeaCucumber::None) => {
                                next.set_at(rx, ry, SeaCucumber::Down);
                                moved = true;
                            },
                            _ => {
                                next.set_at(x, y, SeaCucumber::Down);
                            }
                        }
                    },
                    Some(SeaCucumber::Right) => {
                        next.set_at(x, y, SeaCucumber::Right);
                    },
                    _ => ()
                }
            }
        }

        (next, moved)
    }

    fn _display(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.get_at(x, y) {
                    Some(SeaCucumber::Right) => print!(">"),
                    Some(SeaCucumber::Down) => print!("v"),
                    Some(SeaCucumber::None) => print!("."),
                    _ => ()
                }
            }
            println!("");
        }
    }
}

pub fn run() {
    run_part_01();
}

fn run_part_01() {
    let file = File::open("input/day25.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let lines:Vec<String> = reader.lines().map(|x| x.unwrap()).collect();

    let mut map = Map::new_from_lines(lines);

    let mut moved = true;

    let mut num_steps = 0;
    while moved {
        let (new_map, new_moved) = map.step();
        map = new_map;
        moved = new_moved;
        num_steps += 1;
    }

    println!("After {} steps the sea cucumbers stopped moving", num_steps);
}