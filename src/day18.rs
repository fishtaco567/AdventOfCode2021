use std::fs::File;
use std::io::{BufReader, BufRead};

enum Token {
    Numeric(u32),
    OpenBracket,
}

enum Node {
    Number(u32),
    Pair(Box<Node>, Box<Node>),
}

#[derive(Clone, Copy)]
enum Side {
    Left,
    Right
}

enum ScanOp {
    NoOp,
    Split,
}

enum ExplodeOp {
    NoOp,
    Explode(u32, u32),
    ExplodeLeft(u32),
    ExplodeRight(u32),
    Exploded
}

pub fn run() {
    println!("--Part 1");
    run_part_01();
    println!("--Part 2");
    run_part_02();
}

fn run_part_01() {    
    let file = File::open("input/day18.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    let mut head = into_tree(lines.next().unwrap().expect("Failed to read line"));
    for line in lines {
        let l = line.expect("Failed to read line");

        head = add(head, into_tree(l));
    }

    show(&head);
    println!("");

    let sum = mag(&head);

    println!("The magnitude of this sum is {}", sum);
}

fn run_part_02() {    
    let file = File::open("input/day18.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let lines = reader.lines().map(|x| x.expect("Failed to read line")).collect::<Vec<String>>();

    let mut highest_magnitude = 0;

    for x in 0..lines.len() {
        for y in 0..lines.len() {
            if x == y {
                continue;
            }
            let left = into_tree(lines[x].clone());
            let right = into_tree(lines[y].clone());
            let added = add(left, right);
            let magnitude = mag(&added);
            if magnitude > highest_magnitude {
                highest_magnitude = magnitude;
            }
        }
    }

    println!("The highest magnitude is {}", highest_magnitude);
}

fn show(n: &Node) {
    match n {
        Node::Number(i) => {
            print!("{}", *i);
        },
        Node::Pair(p1, p2) => {
            print!("[");
            show(p1);
            print!(",");
            show(p2);
            print!("]");
        }
    }
}

fn add(left: Node, right: Node) -> Node {
    let mut new_node = Node::Pair(Box::new(left), Box::new(right));

    reduce(&mut new_node);

    new_node
}

fn reduce(node: &mut Node) {
    let mut done = false;

    while !done {
        done = true;

        match scan_explode(node, 0) {
            ExplodeOp::NoOp => (),
            _ => {
                done = false;
                continue;
            }
        }

        match scan_split(node) {
            ScanOp::NoOp => (),
            _ => {
                done = false;
                continue;
            }
        }
    }
}

fn scan_explode(node: &mut Node, depth:i32) -> ExplodeOp {
    let op = match node {
        Node::Number(_) => ExplodeOp::NoOp,
        Node::Pair(p1, p2) => {
            match (p1.as_mut(), p2.as_mut()) {
                (Node::Number(n1), Node::Number(n2)) => {
                    if depth >= 4 {
                        ExplodeOp::Explode(*n1, *n2)
                    } else {
                        ExplodeOp::NoOp
                    }
                },
                _ => {
                    //Left
                    match scan_explode(p1, depth + 1) {
                        ExplodeOp::NoOp => (),
                        ExplodeOp::Explode(n1, n2) => {
                            distribute_into(p2, n2, Side::Left);
                            return ExplodeOp::ExplodeLeft(n1);
                        },
                        ExplodeOp::ExplodeLeft(n1) => {
                            return ExplodeOp::ExplodeLeft(n1);
                        },
                        ExplodeOp::ExplodeRight(n1) => {
                            distribute_into(p2, n1, Side::Left);
                            return ExplodeOp::Exploded;
                        },
                        ExplodeOp::Exploded => return ExplodeOp::Exploded,
                    };

                    //Right
                    match scan_explode(p2, depth + 1) {
                        ExplodeOp::NoOp => (),
                        ExplodeOp::Explode(n1, n2) => {
                            distribute_into(p1, n1, Side::Right);
                            return ExplodeOp::ExplodeRight(n2);
                        },
                        ExplodeOp::ExplodeLeft(n1) => {
                            distribute_into(p1, n1, Side::Right);
                            return ExplodeOp::Exploded;
                        },
                        ExplodeOp::ExplodeRight(n1) => {
                            return ExplodeOp::ExplodeRight(n1);
                        },
                        ExplodeOp::Exploded => return ExplodeOp::Exploded,
                    }
                    ExplodeOp::NoOp
                }
            }
        }
    };

    match op {
        ExplodeOp::Explode(_,_) => {
            *node = Node::Number(0);
        },
        _ => ()
    }

    op
}

fn distribute_into(node: &mut Node, into: u32, side: Side) {
    match node {
        Node::Number(n) => {
            *n += into;
        },
        Node::Pair(p1, p2) => {
            match side {
                Side::Left => {
                    distribute_into(p1, into, side);
                },
                Side::Right => {
                    distribute_into(p2, into, side);
                }
            }
        }
    }
}

fn scan_split(node:&mut Node) -> ScanOp {
    let (op, n) = match node {
        Node::Number(n) => {
            if *n >= 10 {
                (ScanOp::Split, Some(*n))
            } else {
                (ScanOp::NoOp, None)
            }
        },
        Node::Pair(p1, p2) => {
            match scan_split(p1) {
                ScanOp::NoOp => (),
                ScanOp::Split => return ScanOp::Split,
            };
            match scan_split(p2) {
                ScanOp::NoOp => (ScanOp::NoOp, None),
                ScanOp::Split => return ScanOp::Split,
            }
        }
    };

    match op {
        ScanOp::NoOp => ScanOp::NoOp,
        ScanOp::Split => {
            split(node, n.unwrap());
            ScanOp::Split
        }
    }
}

fn split(n: &mut Node, num: u32) {
    let half = (num as f32) / 2.;
    *n = Node::Pair(Box::new(Node::Number(half.floor() as u32)), Box::new(Node::Number(half.ceil() as u32)));
}

fn mag(n: &Node) -> u32 {
    match n {
        Node::Number(i) => {
            *i
        },
        Node::Pair(p1, p2) => {
            3 * mag(p1) + 2 * mag(p2)
        }
    }
}

fn into_tree(input: String) -> Node {
    let c = input.chars();
    let tokens = tokenize(c);
    parse(&mut tokens.iter())
}

fn tokenize(iter_in: impl Iterator<Item = char>) -> Vec<Token> {
    let mut iter = iter_in;

    let mut tokens:Vec<Token> = Vec::new();
    
    while let Some(c) = iter.next() {
        match c {
            c if c.is_numeric() => {
                let mut s = c.to_string();
                while let Some(c2) = iter.next() {
                    if !c2.is_numeric() {
                        break;
                    }
                    s.push(c2);
                }
                tokens.push(Token::Numeric(s.parse::<u32>().expect("Failed to parse number")));
            },
            '[' => {
                tokens.push(Token::OpenBracket);
            }
            _=> ()
        }
    }

    tokens
}

fn parse<'a>(iter: &mut impl Iterator<Item = &'a Token>) -> Node {
    match iter.next() {
        Some(t) => {
            match t {
                Token::Numeric(n) => {
                    Node::Number(*n)
                },
                Token::OpenBracket => {
                    let left = parse(iter);
                    let right = parse(iter);
                    Node::Pair(Box::new(left), Box::new(right))
                }
            }
        },
        None => Node::Number(0)
    }
}