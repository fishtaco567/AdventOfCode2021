use std::fs;

#[derive(Default, Clone)]
struct BingoSpot {
    number: u32,
    checked: bool,
}

#[derive(Default, Clone)]
struct BingoBoard {
    board: [BingoSpot; 25],
    won: bool
}

impl BingoBoard {
    fn new(s: &str) -> Self {
        let mut bboard: [BingoSpot; 25] = Default::default();

        let places = s.split_whitespace();

        for (index, num_str) in places.enumerate() {
            if index < 25 {
                bboard[index].number = num_str.parse().expect("Cannot parse board");
            }
        }

        BingoBoard{ board: bboard, won: false }
    }

    fn get_index(x: u32, y:u32) -> usize {
        (x + y * 5) as usize
    }

    fn check_number(&mut self, num: u32) {
        for spot in self.board.iter_mut() {
            if spot.number == num {
                spot.checked = true;
            }
        }
    }

    fn check_bingo(&mut self) -> bool {
        if self.won {
            return false;
        }

        let mut success = true;

        for i in 0u32..5 { //Check rows
            success = true;
            for j in 0u32..5 {
                let index = BingoBoard::get_index(j, i);
                success &= self.board[index].checked;
            }

            if success {
                break;
            }
        }

        if success {
            self.won = true;
            return true;
        }

        for i in 0u32..5 { //Check columns
            success = true;
            for j in 0u32..5 {
                let index = BingoBoard::get_index(i, j);
                success &= self.board[index].checked;
            }

            if success {
                break;
            }
        }

        if success {
            self.won = true;
        }

        success
    }

    fn sum_unmarked(&self) -> u32 {
        self.board.iter().fold(0, |acc, x| if x.checked { acc } else { acc + x.number })
    }
}

pub fn run() {
    println!("--Part 1");
    run_part_01();
    println!("--Part 2");
    run_part_02();
}

fn run_part_01() {
    let contents = fs::read_to_string("input/day04.txt").expect("Failed to read file");

    let chunks = contents.split("\r\n\r\n").collect::<Vec<&str>>();

    let sequence = chunks[0].clone().split(',').map(|x| x.parse::<u32>().expect("Failed to parse sequence")).collect::<Vec<u32>>();

    let mut boards: Vec<BingoBoard> = Vec::new();

    for chunk in chunks[1..].iter() {
        boards.push(BingoBoard::new(chunk));
    }

    let mut winner = 0usize;
    let mut last_called = 0u32;

    'outer:
    for num in sequence {
        for (i, board) in boards.iter_mut().enumerate() {
            board.check_number(num);
            if board.check_bingo() {
                winner = i;
                last_called = num;
                break 'outer;
            }
        }
    }

    let sum = boards[winner].sum_unmarked();
    let product = sum * last_called;

    println!("The winning final score is {}", product);
}

fn run_part_02() {
    let contents = fs::read_to_string("input/day04.txt").expect("Failed to read file");

    let chunks = contents.split("\r\n\r\n").collect::<Vec<&str>>();

    let sequence = chunks[0].clone().split(',').map(|x| x.parse::<u32>().expect("Failed to parse sequence")).collect::<Vec<u32>>();

    let mut boards: Vec<BingoBoard> = Vec::new();

    for chunk in chunks[1..].iter() {
        boards.push(BingoBoard::new(chunk));
    }

    let mut loser = 0usize;
    let mut last_called = 0u32;
    let num_boards = boards.len();

    'outer:
    for num in sequence {
        let cur_winners = boards.iter().fold(0, |acc, x| if x.won { acc + 1 } else { acc } );

        for (i, board) in boards.iter_mut().enumerate() {
            board.check_number(num);
            let just_won = board.check_bingo();
            if just_won && cur_winners == num_boards - 1 {
                loser = i;
                last_called = num;
                break 'outer;
            }
        }
    }

    let sum = boards[loser].sum_unmarked();
    let product = sum * last_called;

    println!("The losing final score is {}", product);
}