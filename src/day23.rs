use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::hash::Hash;

#[derive(PartialEq, Eq, Clone)]
struct State {
    amphipods: HashMap<usize, Amphipod>,
    score: i32
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq, Clone, Hash)]
struct Amphipod {
    cell: usize,
    kind: char,
    id: usize
}

impl Amphipod {
    fn is_home(&self, board: &Board) -> bool {
        match board.cells.get(self.cell) {
            Some(Cell::Room(c,_)) => *c == self.kind,
            Some(Cell::RoomEnd(c)) => *c == self.kind,
            _ => false
        }
    }
}

#[derive(PartialEq, Eq)]
enum Cell {
    Room(char, usize),
    RoomEnd(char),
    Hall,
    Tee
}

struct Board {
    connections: HashMap<usize, Vec<usize>>,
    cells: Vec<Cell>,
}

impl Board {
    fn new() -> Self {
        Board { connections: HashMap::new(), cells: Vec::new(), }
    }

    fn add_cell(&mut self, kind: Cell, predecessors: Vec<usize>) -> usize {
        let id = self.cells.len();
        self.cells.push(kind);

        for p in predecessors {
            self.connections.entry(id).or_insert(Vec::new()).push(p);
            self.connections.entry(p).or_insert(Vec::new()).push(id);
        }

        id
    }

    fn _display(&self) {
        for (id, cell) in self.cells.iter().enumerate() {
            let s:String = match cell {
                Cell::Room(c,_) => {
                    let mut r = "Room ".to_string();
                    r.push(*c);
                    r
                },
                Cell::RoomEnd(c) => {
                    let mut r = "RoomEnd ".to_string();
                    r.push(*c);
                    r
                },
                Cell::Hall => "Hall".to_string(),
                Cell::Tee => "Tee".to_string(),
            };

            let connections = self.connections.get(&id).unwrap();

            if connections.len() == 1 {
                println!("Cell {} at ID {}, connecting to {}", s, id, connections[0]);
            } else if connections.len() == 2 {
                println!("Cell {} at ID {}, connecting to {} and {}", s, id, connections[0], connections[1]);
            } else if connections.len() == 3 {
                println!("Cell {} at ID {}, connecting to {} and {} and {}", s, id, connections[0], connections[1], connections[2]);
            }
        }
    }

    fn _display_amphis(&self, amphis: &State) {
        let indicies_zero = vec![0,1,2,7,8,13,14,19,20,25,26];
        let indicies_next1 = vec![-1,-2,3,-2,9,-2,15,-2,21,-2,-1];
        let indicies_next2 = vec![-1,-2,4,-2,10,-2,16,-2,22,-2,-1];
        let indicies_next3 = vec![-1,-2,5,-2,11,-2,17,-2,23,-2,-1];
        let indicies_next4 = vec![-1,-2,6,-2,12,-2,18,-2,24,-2,-1];

        let mut occupied = HashMap::new();
        let _ = &amphis.amphipods.values().for_each(|a| { occupied.insert(a.cell, a.kind); });
        
        println!(".............");
        for y in 0..5 {
            print!(".");
            for x in 0..11 {
                let i:i32 = match y {
                    0 => indicies_zero[x],
                    1 => indicies_next1[x],
                    2 => indicies_next2[x],
                    3 => indicies_next3[x],
                    4 => indicies_next4[x],
                    _ => -1
                };

                if i == -1 {
                    print!(".");
                } else if i == -2 {
                    print!("|");
                } else {
                    let q = occupied.get(&(i as usize));
                    match q {
                        Some(x) => print!("{}", x),
                        None => print!(" ")
                    }
                }
            }
            println!(".");
        }
        println!(".............");
    }
}

pub fn run() {
    println!("--Part 1");
    run_part_01();
    println!("--Part 2");
    run_part_02();
}

fn run_part_01() {
    let (board, amphipods) = setup_part_01();

    //board.display();

    let orig_state = State { amphipods, score: 0 };

    let result = simulate(&board, orig_state);

    match result {
        Some(x) => {
            println!("The minimum score is {}", x.score);
        },
        None => ()
    }
}

fn run_part_02() {
    let (board, amphipods) = setup_part_02();

    //board.display();

    let orig_state = State { amphipods, score: 0 };

    let result = simulate(&board, orig_state);

    match result {
        Some(x) => {
            println!("The minimum score is {}", x.score);
        },
        None => ()
    }
}

fn simulate(board: &Board, orig_state: State) -> Option<State> {
    let mut queue = BinaryHeap::new();

    let mut explored:HashMap<Vec<Amphipod>, i32> = HashMap::new();
    explored.insert(orig_state.amphipods.values().map(|x| x.clone()).collect::<Vec<Amphipod>>(), orig_state.score.clone());

    queue.push(orig_state);

    let mut t = std::time::Instant::now();
    while let Some(next) = queue.pop() {
        if t.elapsed().as_secs() > 2 {
            t = std::time::Instant::now();
            //println!("{}", next.score);
            //board.display_amphis(&next);
        }

        let mut is_done = true;
        for amphi in &next.amphipods {
            if !amphi.1.is_home(&board) {
                is_done = false;
            }
        }

        if is_done {
            return Some(next);
        }

        let succ = successors(&next, &board);

        for s in succ {
            let amphis = s.amphipods.values().map(|x| x.clone()).collect::<Vec<Amphipod>>();
            if explored.contains_key(&amphis) && *explored.get(&amphis).unwrap() <= s.score {
                continue;
            } else {
                explored.insert(amphis, s.score);
            }

            queue.push(s);
        }
    }
    None
}

fn successors(state: &State, board: &Board) -> Vec<State> {
    let mut out = Vec::new();
    
    let mut occupied = HashMap::new();
    let _ = &state.amphipods.values().for_each(|a| { occupied.insert(a.cell, a.kind); });

    for amphi in &mut state.amphipods.values() {
        let mut visited = HashSet::new();

        let cell = &board.cells[amphi.cell];
        if let Cell::RoomEnd(c) = cell {
            if *c == amphi.kind {
                continue;
            }
        }

        if let Cell::Room(c, next) = cell {
            let mut n = *next;
            if *c == amphi.kind {
                let mut all_match = true;
                while occupied.contains_key(&n) {
                    if !(*occupied.get(&n).unwrap() == amphi.kind) {
                        all_match = false;
                    }

                    if let Some(Cell::Room(_,i)) = &board.cells.get(n) {
                        n = *i;
                    } else {
                        break;
                    }
                }

                if all_match {
                    continue;
                }
            }
        }

        find_all_valid(amphi, state, board, cell, &occupied, &mut out, &mut visited, 0);
    }

    out
}

fn find_all_valid(amphi: &Amphipod, state: &State, board: &Board, coming_from: &Cell, occupied: &HashMap<usize, char>, valid: &mut Vec<State>, visited: &mut HashSet<usize>, score: i32) {
    let mut score_add = score;
    
    score_add += match amphi.kind {
        'A' | 'a' => 1,
        'B' | 'b' => 10,
        'C' | 'c' => 100,
        'D' | 'd' => 1000,
        _ => 0
    };

    if visited.contains(&amphi.cell) {
        return;
    }

    visited.insert(amphi.cell);

    for connected in board.connections.get(&amphi.cell).unwrap() {
        if occupied.contains_key(connected) {
            continue;
        }

        match coming_from {
            Cell::Room(_, _) | Cell::RoomEnd(_) => {
                match &board.cells[*connected] {
                    Cell::Room(_,_) => {
                        find_all_valid(&Amphipod { cell: *connected, kind: amphi.kind, id: amphi.id }, state, board, coming_from, occupied, valid, visited, score_add);
                    },
                    Cell::RoomEnd(_) => {
                        find_all_valid(&Amphipod { cell: *connected, kind: amphi.kind, id: amphi.id }, state, board, coming_from, occupied, valid, visited, score_add);
                    },
                    Cell::Hall => {
                        let mut new = state.clone();
                        new.amphipods.entry(amphi.id).or_insert(Amphipod {kind: '_', cell:0, id: amphi.id }).cell = *connected;
                        new.score += score_add;
                        valid.push(new);
                        find_all_valid(&Amphipod { cell: *connected, kind: amphi.kind, id: amphi.id }, state, board, coming_from, occupied, valid, visited, score_add);
                    },
                    Cell::Tee => {
                        find_all_valid(&Amphipod { cell: *connected, kind: amphi.kind, id: amphi.id }, state, board, coming_from, occupied, valid, visited, score_add);
                    },
                }
            },
            Cell::Hall => {
                match &board.cells[*connected] {
                    Cell::Room(c, next) => {
                        if *c != amphi.kind {
                            continue;
                        }

                        if occupied.contains_key(connected) {
                            continue;
                        }

                        let mut all_match = false;
                        if occupied.contains_key(next) {
                            all_match = true;
                            let mut n = *next;
                            while occupied.contains_key(&n) {
                                if !(*occupied.get(&n).unwrap() == amphi.kind) {
                                    all_match = false;
                                }
            
                                if let Some(Cell::Room(_,i)) = &board.cells.get(n) {
                                    n = *i;
                                } else {
                                    break;
                                }
                            }
                        } else {
                            find_all_valid(&Amphipod { cell: *connected, kind: amphi.kind, id: amphi.id }, state, board, coming_from, occupied, valid, visited, score_add);
                        }

                        if all_match {
                            let mut new = state.clone();
                            new.amphipods.entry(amphi.id).or_insert(Amphipod {kind: '_', cell:0, id: amphi.id }).cell = *connected;
                            new.score += score_add;
                            valid.push(new);
                        }
                    },
                    Cell::RoomEnd(c) => {
                        if *c != amphi.kind {
                            continue;
                        }

                        if !occupied.contains_key(connected) {
                            let mut new = state.clone();
                            new.amphipods.entry(amphi.id).or_insert(Amphipod {kind: '_', cell:0, id: amphi.id }).cell = *connected;
                            new.score += score_add;
                            valid.push(new);
                        }
                    },
                    Cell::Hall => {
                        find_all_valid(&Amphipod { cell: *connected, kind: amphi.kind, id: amphi.id }, state, board, coming_from, occupied, valid, visited, score_add);
                    },
                    Cell::Tee => {
                        find_all_valid(&Amphipod { cell: *connected, kind: amphi.kind, id: amphi.id }, state, board, coming_from, occupied, valid, visited, score_add);
                    },
                }
            },
            _ => (),
        }
    }
}

fn _setup_test() -> (Board, HashMap<usize, Amphipod>) {
    let mut board = Board::new();
    let mut amphipods = HashMap::new();

    let id1 = board.add_cell(Cell::Hall, Vec::new());
    let id2 = board.add_cell(Cell::Hall, vec![id1]);

    let id3 = board.add_cell(Cell::Tee, vec![id2]);
    let room1 = board.add_cell(Cell::Room('A', id3 + 2), vec![id3]);
    amphipods.insert(0, Amphipod { cell: room1, kind: 'B', id: 0 });
    let room11 = board.add_cell(Cell::RoomEnd('A'), vec![room1]);
    amphipods.insert(1, Amphipod { cell: room11, kind: 'A', id: 1 });

    let id4 = board.add_cell(Cell::Hall, vec![id3]);
    let id5 = board.add_cell(Cell::Tee, vec![id4]);
    let room2 = board.add_cell(Cell::Room('B', id5 + 2), vec![id5]);
    amphipods.insert(2, Amphipod { cell: room2, kind: 'C', id: 2 });
    let room21 = board.add_cell(Cell::RoomEnd('B'), vec![room2]);
    amphipods.insert(3, Amphipod { cell: room21, kind: 'D', id: 3 });

    let id6 = board.add_cell(Cell::Hall, vec![id5]);
    let id7 = board.add_cell(Cell::Tee, vec![id6]);
    let room3 = board.add_cell(Cell::Room('C', id7 + 2), vec![id7]);
    amphipods.insert(4, Amphipod { cell: room3, kind: 'B', id: 4 });
    let room31 = board.add_cell(Cell::RoomEnd('C'), vec![room3]);
    amphipods.insert(5, Amphipod { cell: room31, kind: 'C', id: 5 });

    let id8 = board.add_cell(Cell::Hall, vec![id7]);
    let id9 = board.add_cell(Cell::Tee, vec![id8]);
    let room4 = board.add_cell(Cell::Room('D', id9 + 2), vec![id9]);
    amphipods.insert(6, Amphipod { cell: room4, kind: 'D', id: 6 });
    let room41 = board.add_cell(Cell::RoomEnd('D'), vec![room4]);
    amphipods.insert(7, Amphipod { cell: room41, kind: 'A', id: 7 });

    let id10 = board.add_cell(Cell::Hall, vec![id9]);
    let _ = board.add_cell(Cell::Hall, vec![id10]);

    (board, amphipods)
}

fn setup_part_01() -> (Board, HashMap<usize, Amphipod>) {
    let mut board = Board::new();
    let mut amphipods = HashMap::new();

    let id1 = board.add_cell(Cell::Hall, Vec::new());
    let id2 = board.add_cell(Cell::Hall, vec![id1]);

    let id3 = board.add_cell(Cell::Tee, vec![id2]);
    let room1 = board.add_cell(Cell::Room('A', id3 + 2), vec![id3]);
    amphipods.insert(0, Amphipod { cell: room1, kind: 'B', id: 0 });
    let room11 = board.add_cell(Cell::RoomEnd('A'), vec![room1]);
    amphipods.insert(1, Amphipod { cell: room11, kind: 'B', id: 1 });

    let id4 = board.add_cell(Cell::Hall, vec![id3]);
    let id5 = board.add_cell(Cell::Tee, vec![id4]);
    let room2 = board.add_cell(Cell::Room('B', id5 + 2), vec![id5]);
    amphipods.insert(2, Amphipod { cell: room2, kind: 'A', id: 2 });
    let room21 = board.add_cell(Cell::RoomEnd('B'), vec![room2]);
    amphipods.insert(3, Amphipod { cell: room21, kind: 'C', id: 3 });

    let id6 = board.add_cell(Cell::Hall, vec![id5]);
    let id7 = board.add_cell(Cell::Tee, vec![id6]);
    let room3 = board.add_cell(Cell::Room('C', id7 + 2), vec![id7]);
    amphipods.insert(4, Amphipod { cell: room3, kind: 'A', id: 4 });
    let room31 = board.add_cell(Cell::RoomEnd('C'), vec![room3]);
    amphipods.insert(5, Amphipod { cell: room31, kind: 'D', id: 5 });

    let id8 = board.add_cell(Cell::Hall, vec![id7]);
    let id9 = board.add_cell(Cell::Tee, vec![id8]);
    let room4 = board.add_cell(Cell::Room('D', id9 + 2), vec![id9]);
    amphipods.insert(6, Amphipod { cell: room4, kind: 'D', id: 6 });
    let room41 = board.add_cell(Cell::RoomEnd('D'), vec![room4]);
    amphipods.insert(7, Amphipod { cell: room41, kind: 'C', id: 7 });

    let id10 = board.add_cell(Cell::Hall, vec![id9]);
    let _ = board.add_cell(Cell::Hall, vec![id10]);

    (board, amphipods)
}

fn setup_part_02() -> (Board, HashMap<usize, Amphipod>) {
    let mut board = Board::new();
    let mut amphipods = HashMap::new();

    let id1 = board.add_cell(Cell::Hall, Vec::new());
    let id2 = board.add_cell(Cell::Hall, vec![id1]);

    let id3 = board.add_cell(Cell::Tee, vec![id2]);
    let room1 = board.add_cell(Cell::Room('A', id3 + 2), vec![id3]);
    amphipods.insert(0, Amphipod { cell: room1, kind: 'B', id: 0 });
    let room11 = board.add_cell(Cell::Room('A', room1 + 2), vec![room1]);
    amphipods.insert(1, Amphipod { cell: room11, kind: 'D', id: 1 });
    let room12 = board.add_cell(Cell::Room('A', room11 + 2), vec![room11]);
    amphipods.insert(2, Amphipod { cell: room12, kind: 'D', id: 2 });
    let room13 = board.add_cell(Cell::RoomEnd('A'), vec![room12]);
    amphipods.insert(3, Amphipod { cell: room13, kind: 'B', id: 3 });

    let id4 = board.add_cell(Cell::Hall, vec![id3]);
    let id5 = board.add_cell(Cell::Tee, vec![id4]);
    let room2 = board.add_cell(Cell::Room('B', id5 + 2), vec![id5]);
    amphipods.insert(4, Amphipod { cell: room2, kind: 'A', id: 4 });
    let room21 = board.add_cell(Cell::Room('B', room2 + 2), vec![room2]);
    amphipods.insert(5, Amphipod { cell: room21, kind: 'C', id: 5 });
    let room22 = board.add_cell(Cell::Room('B', room21 + 2), vec![room21]);
    amphipods.insert(6, Amphipod { cell: room22, kind: 'B', id: 6 });
    let room23 = board.add_cell(Cell::RoomEnd('B'), vec![room22]);
    amphipods.insert(7, Amphipod { cell: room23, kind: 'C', id: 7 });

    let id6 = board.add_cell(Cell::Hall, vec![id5]);
    let id7 = board.add_cell(Cell::Tee, vec![id6]);
    let room3 = board.add_cell(Cell::Room('C', id7 + 2), vec![id7]);
    amphipods.insert(8, Amphipod { cell: room3, kind: 'A', id: 8 });
    let room31 = board.add_cell(Cell::Room('C', room3 + 2), vec![room3]);
    amphipods.insert(9, Amphipod { cell: room31, kind: 'B', id: 9 });
    let room32 = board.add_cell(Cell::Room('C', room31 + 2), vec![room31]);
    amphipods.insert(10, Amphipod { cell: room32, kind: 'A', id: 10 });
    let room33 = board.add_cell(Cell::RoomEnd('C'), vec![room32]);
    amphipods.insert(11, Amphipod { cell: room33, kind: 'D', id: 11 });

    let id8 = board.add_cell(Cell::Hall, vec![id7]  );
    let id9 = board.add_cell(Cell::Tee, vec![id8]);
    let room4 = board.add_cell(Cell::Room('D', id9 + 2), vec![id9]);
    amphipods.insert(12, Amphipod { cell: room4, kind: 'D', id: 12 });
    let room41 = board.add_cell(Cell::Room('D', room4 + 2), vec![room4]);
    amphipods.insert(13, Amphipod { cell: room41, kind: 'A', id: 13 });
    let room42 = board.add_cell(Cell::Room('D', room41 + 2), vec![room41]);
    amphipods.insert(14, Amphipod { cell: room42, kind: 'C', id: 14 });
    let room43 = board.add_cell(Cell::RoomEnd('D'), vec![room42]);
    amphipods.insert(15, Amphipod { cell: room43, kind: 'C', id: 15 });

    let id10 = board.add_cell(Cell::Hall, vec![id9]);
    let _ = board.add_cell(Cell::Hall, vec![id10]);

    (board, amphipods)
}