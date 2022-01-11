use std::{fs, path::Iter, str::Chars};

enum Packet {
    Literal(u8, u8, u64),
    Operator(u8, u8, u8, u32),
    None
}

struct ParseNode {
    packet_type:Packet,
    children: Vec<ParseNode>
}

pub fn run() {
    run_part_01();
}

fn run_part_01() {
    let input:String = fs::read_to_string("input/day16.txt").expect("Failed to read file");

    let parse_string = input.chars().map(|c| to_binary(c)).collect::<String>();
    let mut parse_stream = parse_string.chars();

    let mut head = ParseNode { packet_type: Packet::None, children: Vec::new() };

    parse(&mut head, &mut parse_stream, 0);

    let version_sum = add_versions(&head);

    let outer = evaluate(&head.children[0]);
    
    println!("The version sum is {}", version_sum);
    println!("The expression evaluates to {}", outer);
}

fn parse(parent: &mut ParseNode, stream: &mut Chars, indent:i32) -> u32 {
    let ind_str = (0..indent).map(|_| "    ").collect::<String>();

    let mut bits = 0;
    
    let ver = u8::from_str_radix(stream.take(3).collect::<String>().as_str(), 2).expect("Failed to parse version");
    let id = u8::from_str_radix(stream.take(3).collect::<String>().as_str(), 2).expect("Failed to parse id");

    bits += 6;

    match id {
        4 => {
            let (parsed_bits, literal) = parse_literal(stream);
            bits += parsed_bits;
            parent.children.push(ParseNode { packet_type: Packet::Literal(ver, id, literal), children: Vec::new() });
            println!("{}Parsed literal ver: {} id: {} num: {}", ind_str, ver, id, literal);
        },
        _ => {
            let len_id = stream.next().unwrap().to_digit(2).unwrap() as u8;

            bits += 1;

            let len = match len_id {
                0 => {
                    bits += 15;
                    u32::from_str_radix(stream.take(15).collect::<String>().as_str(),2).expect("Failed to parse length")
                },
                1 => {
                    bits += 11;
                    u32::from_str_radix(stream.take(11).collect::<String>().as_str(),2).expect("Failed to parse length")
                }
                _ => 0
            };

            parent.children.push(ParseNode { packet_type: Packet::Operator(ver, id, len_id, len), children: Vec::new()});
            println!("{}Parsed op ver: {} id: {} type: {} len: {}", ind_str, ver, id, len_id, len);
            match len_id {
                0 => {
                    let mut bits_parsed = 0;
                    while bits_parsed < len {
                        bits_parsed += parse(parent.children.last_mut().unwrap(), stream, indent + 1);
                    }

                    bits += bits_parsed;
                },
                1 => {
                    for _ in 0..len {
                        bits += parse(parent.children.last_mut().unwrap(), stream, indent + 1);
                    }
                },
                _ => ()
            }
        }
    }

    bits
}

fn parse_literal(stream: &mut Chars) -> (u32, u64) {
    let mut bits = 0;

    let mut binary = String::new();

    while stream.next() == Some('1') {
        for _ in 0..4 {
            if let Some(c) = stream.next() {
                binary.push(c);
            }
        }

        bits += 5;
    }
    
    for _ in 0..4 {
        if let Some(c) = stream.next() {
            binary.push(c);
        }
    }

    bits += 5;

    (bits, u64::from_str_radix(binary.as_str(), 2).expect("Failed to parse literal"))
}

fn add_versions(node: &ParseNode) -> u32 {
    let mut self_ver = match node.packet_type {
        Packet::Literal(ver, id, num) => ver,
        Packet::Operator(ver, id, len_id, len) => ver,
        Packet::None => 0
    } as u32;

    for child in &node.children {
        self_ver += add_versions(child);
    }

    self_ver
}

fn evaluate(node: &ParseNode) -> u64 {
    match node.packet_type {
        Packet::Literal(ver, id, num) => num,
        Packet::Operator(ver, id, len_id, len) => {
            match id {
                0 => {
                    node.children.iter().fold(0, |acc, x| acc + evaluate(x))
                },
                1 => {
                    node.children.iter().fold(1, |acc, x| acc * evaluate(x))
                },
                2 => {
                    node.children.iter().fold(u64::MAX, |acc, x| acc.min(evaluate(x)))
                },
                3 =>  {
                    node.children.iter().fold(u64::MIN, |acc, x| acc.max(evaluate(x)))
                },
                5 => {
                    assert_eq!(node.children.len(), 2);

                    if evaluate(&node.children[0]) > evaluate(&node.children[1]) {
                        1
                    } else {
                        0
                    }
                },
                6 => {
                    assert_eq!(node.children.len(), 2);

                    if evaluate(&node.children[0]) < evaluate(&node.children[1]) {
                        1
                    } else {
                        0
                    }
                },
                7 => {
                    assert_eq!(node.children.len(), 2);

                    if evaluate(&node.children[0]) == evaluate(&node.children[1]) {
                        1
                    } else {
                        0
                    }
                },
                _ => 0,
            }
        },
        Packet::None => 0
    }
}

fn to_binary(c: char) -> &'static str {
    match c.to_ascii_uppercase() {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "0000",
    }
}