use std::fs::File;
use std::hash::Hash;
use std::io::{BufReader, BufRead};
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Copy, Clone, Eq, PartialEq)]
struct ActiveNode {
    estimated_cost:i32,
    pos:(i32, i32),
}

impl Ord for ActiveNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.estimated_cost.cmp(&self.estimated_cost).then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for ActiveNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct Neighbors {
    pos: (i32, i32),
    max: (i32, i32),
    num: i32
}

impl Neighbors {
    fn new(at: (i32, i32), with_max:(i32, i32)) -> Self {
        Neighbors { pos: at, max: with_max, num: 0}
    }
}

impl Iterator for Neighbors {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<(i32, i32)> {
        match self.num {
            0 => {
                self.num += 1;
                if self.pos.0 > 0 {
                    Some(((self.pos.0 - 1), self.pos.1))
                } else {
                    self.next()
                }
            },
            1 => {
                self.num += 1;
                if self.pos.0 < (self.max.0 - 1) {
                    Some(((self.pos.0 + 1), self.pos.1))
                } else {
                    self.next()
                }
            },
            2 => {
                self.num += 1;
                if self.pos.1 > 0 {
                    Some((self.pos.0, (self.pos.1 - 1)))
                } else {
                    self.next()
                }
            },
            3 => {
                self.num += 1;
                if self.pos.1 < (self.max.1 - 1) {
                    Some((self.pos.0, (self.pos.1 + 1)))
                } else {
                    None
                }
            },
            _ => None
        }
    }
}

struct CostView<'a> {
    cost: &'a Vec<i32>,
    real_size: (i32, i32),
    repeat: (i32, i32)
}

impl<'a> CostView<'a> {
    fn new(cost:&'a Vec<i32>, size:(i32, i32), repeat:(i32, i32)) -> Self {
        CostView {
            cost: cost,
            real_size: size,
            repeat: repeat
        }
    }

    fn value_at(&self, pos:(i32, i32)) -> Option<i32> {
        if pos.0 < 0 || pos.1 < 0 || pos.0 >= self.real_size.0 * self.repeat.0 || pos.1 >= self.real_size.1 * self.repeat.1 {
            return None
        }

        let inc = 0 + (pos.0 / self.real_size.0) + (pos.1 / self.real_size.1);
        let red_pos = (pos.0 % self.real_size.0, pos.1 % self.real_size.1);
        let index = index_of(&red_pos, &self.real_size);
        let mut num = inc + self.cost[index];
        while num > 9 {
            num = num - 9;
        }
        Some(num)
    }

    fn get_max_size(&self) -> (i32, i32) {
        (self.real_size.0 * self.repeat.0, self.real_size.1 * self.repeat.1)
    }

    fn get_max_len(&self) -> i32 {
        (self.real_size.0 * self.repeat.0) * (self.real_size.1 * self.repeat.1)
    }

    fn display(&self) {
        let s = self.get_max_size();

        for y in 0..s.1 {
            for x in 0..s.0 {
                print!("{}", self.value_at((x, y)).unwrap());
            }
            println!("");
        }
    }

    fn display_with(&self, path:&Vec<(i32, i32)>) {
        let mut set:HashSet<(i32, i32)> = HashSet::new();
        for point in path {
            set.insert(point.to_owned());
        }

        let s = self.get_max_size();

        for y in 0..s.1 {
            for x in 0..s.0 {
                if set.contains(&(x, y)) {
                    print!("{}", self.value_at((x, y)).unwrap());
                } else {
                    print!(".");
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
    let file = File::open("input/day15.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut size:(i32, i32) = (0, 0);

    let mut cost:Vec<i32> = Vec::new();

    for line in reader.lines() {
        let l = line.expect("Failed to read line");

        size.0 = l.len() as i32;
        size.1 += 1;

        for c in l.chars() {
            cost.push(c.to_digit(10).expect("Failed to parse digit") as i32);
        }
    }

    let cv = CostView::new(&cost, size, (1, 1));
    let path = path_to(&cv, &size, &(0, 0), &(size.0 - 1, size.1 - 1));

    if let Some(p) = path {        
        let sum = p.iter().fold(0, |acc, x| acc + cost[index_of(x, &size)]) - cost[index_of(&(0, 0), &size)];
        println!("The total cost of this path is {}", sum);
    }
}


fn run_part_02() {
    let file = File::open("input/day15.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut size:(i32, i32) = (0, 0);

    let mut cost:Vec<i32> = Vec::new();

    for line in reader.lines() {
        let l = line.expect("Failed to read line");

        size.0 = l.len() as i32;
        size.1 += 1;

        for c in l.chars() {
            cost.push(c.to_digit(10).expect("Failed to parse digit") as i32);
        }
    }

    let cv = CostView::new(&cost, size, (5, 5));
    let path = path_to(&cv, &cv.get_max_size(), &(0, 0), &(size.0 * 5 - 1, size.1 * 5 - 1));

    if let Some(p) = path {   
        let sum = p.iter().fold(0, |acc, x| acc + cv.value_at(x.to_owned()).unwrap()) - cost[index_of(&(0, 0), &size)];
        println!("The total cost of this five-times path is {}", sum);
    }
}

fn path_to(cost: &CostView, size: &(i32, i32), start:&(i32, i32), end:&(i32, i32)) -> Option<Vec<(i32, i32)>> {
    let mut distance:Vec<i32> = (0..cost.get_max_len()).map(|_| i32::MAX).collect();

    let mut open_set:BinaryHeap<ActiveNode> = BinaryHeap::new();

    open_set.push(ActiveNode { estimated_cost: dist_est(start, &end), pos: start.to_owned()});

    let mut connections:HashMap<(i32, i32), (i32, i32)> = HashMap::new();

    distance[0] = 0;

    while let Some(node) = open_set.pop() {
        if node.pos == *end {
            return Some(reconstruct_path(node.pos, connections))
        }

        let start_distance = distance[index_of(&node.pos, &size)];

        for p in Neighbors::new(node.pos, cost.get_max_size()) {
            let neighbor_index = index_of(&p, &size);
            if let Some(next_cost) = cost.value_at(p) {
                let possible_distance = start_distance + next_cost;
    
                if possible_distance < distance[neighbor_index] {
                    *connections.entry(p).or_default() = node.pos;
    
                    distance[neighbor_index] = possible_distance;
    
                    open_set.push(ActiveNode{ estimated_cost: possible_distance + dist_est(&p, &end), pos: p });
                }
            }
        }
    }

    None
}

fn reconstruct_path(pos: (i32, i32), connections: HashMap<(i32, i32), (i32, i32)>) -> Vec<(i32, i32)> {
    let mut path = Vec::new();
    path.push(pos);

    let mut head = &pos;

    while let Some(next) = connections.get(head) {
        path.push(next.to_owned());
        head = next;
    }

    path
}

fn dist_est(from: &(i32, i32), to: &(i32, i32)) -> i32 {
    (from.0 - to.0).abs() + (from.1 - to.1).abs()
}

fn index_of(pos: &(i32, i32), size: &(i32, i32)) -> usize {
    (pos.0 + pos.1 * size.0) as usize
}