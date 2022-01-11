use std::{collections::HashMap, fs::File, io::{BufReader, BufRead}};

type Coord = (i32, i32);

struct Image {
    image: HashMap<Coord, char>,
    min: Coord,
    max: Coord,
    overmin: Coord,
    overmax: Coord,
}

impl Image {
    fn new(data: &Vec<char>, mini: Coord, maxi: Coord, over:i32) -> Self {
        let mut image = Image { image: HashMap::new(), min: mini, max: maxi, overmin: (mini.0 - over, mini.1 - over), overmax: (maxi.0 + over, maxi.1 + over) };
        
        let width = maxi.0 - mini.0;

        for i in image.min.0..image.max.0 {
            for j in image.min.1..image.max.1 {
                let coord = (i, j);
                image.image.insert(coord, data[(i + j * width) as usize]);
            }
        }

        for i in image.overmin.0..image.overmax.0 {
            for j in image.overmin.1..image.overmax.1 {
                let coord = (i, j);
                image.image.entry(coord).or_insert('0');
            }
        }

        image
    }
    
    fn get_at(&self, x: &Coord) -> char {
        match self.image.get(x) {
            Some(n) => *n,
            None => '0',
        }
    }

    fn get_lookup_for(&self, x: &Coord) -> usize {
        let mut s = String::new();
        
        for k in (x.1 - 1)..=(x.1 + 1_) {
            for i in (x.0 - 1)..=(x.0 + 1) {
                s.push(self.get_at(&(i, k)));
            }
        }

        //println!("{}", s);
        return usize::from_str_radix(&s, 2).expect("Failed to parse lookup")
    }

    fn enhance(&mut self, enhancement_alg: &Vec<char>) {
        let mut new = HashMap::new();

        for i in self.overmin.0..self.overmax.0 {
            for j in self.overmin.1..self.overmax.1 {
                let lookup = self.get_lookup_for(&(i, j));
                new.insert((i, j), enhancement_alg[lookup]);
            }
        }

        self.min = (self.min.0 - 1, self.min.1 - 1);
        self.max = (self.max.0 + 1, self.max.1 + 1);
        self.image = new;
    }

    fn count(&self) -> i32 {
        let mut num = 0;
        for i in self.min.0..self.max.0 {
            for j in self.min.1..self.max.1 {
                if self.get_at(&(i, j)) == '1' {
                    num += 1;
                }    
            }
        }

        num
    }

    fn _display(&self) {
        for j in self.min.1..self.max.1 {
            for i in self.min.0..self.max.0 {
                print!("{}", match self.get_at(&(i, j))
            {
                '1' => '#',
                '0' => '.',
                _ => '.'
            });
            }
            println!("");
        }
        println!("");
    }
}

pub fn run() {
    println!("--Part 1");
    run_part_01();
    println!("--Part 2");
    run_part_02();
}

fn run_part_01() {
    let file = File::open("input/day20.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    let enhancement_alg:Vec<char> = lines.next().unwrap().unwrap().chars().map(|c|
        match c {
            '#' => '1',
            '.' => '0',
            _ => '0'
        }
    ).collect();

    lines.next(); //skip

    let mut image_data:Vec<char> = Vec::new();
    let min = (0, 0);
    let mut max = (0, 0);

    for line in lines {
        let l = line.unwrap();

        max.0 = l.len() as i32;
        max.1 += 1;

        let chars = l.chars().map(|c|
            match c {
                '#' => '1',
                '.' => '0',
                _ => '0'
            }
        );
        for c in chars {
            image_data.push(c);
        }
    }

    let mut image = Image::new(&image_data, min, max, 5);

    for _ in 0..2 {
        image.enhance(&enhancement_alg);
    }

    let num = image.count();

    println!("There were {} lit pixels after twice enhancement", num);
}

fn run_part_02() {
    let file = File::open("input/day20.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    let enhancement_alg:Vec<char> = lines.next().unwrap().unwrap().chars().map(|c|
        match c {
            '#' => '1',
            '.' => '0',
            _ => '0'
        }
    ).collect();

    lines.next(); //skip

    let mut image_data:Vec<char> = Vec::new();
    let min = (0, 0);
    let mut max = (0, 0);

    for line in lines {
        let l = line.unwrap();

        max.0 = l.len() as i32;
        max.1 += 1;

        let chars = l.chars().map(|c|
            match c {
                '#' => '1',
                '.' => '0',
                _ => '0'
            }
        );
        for c in chars {
            image_data.push(c);
        }
    }

    let mut image = Image::new(&image_data, min, max, 100);

    for _ in 0..50 {
        image.enhance(&enhancement_alg);
    }

    let num = image.count();

    println!("There were {} lit pixels after 50x enhancement", num);
}