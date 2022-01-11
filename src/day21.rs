use std::{collections::{HashMap, HashSet}, hash::Hash};

struct DeterministicRoller {
    state:i32,
    num_rolled:i32
}

impl DeterministicRoller {
    fn new() -> Self {
        DeterministicRoller{ state: 0, num_rolled:0 }
    }
    
    fn roll(&mut self) -> i32 {
        self.state += 1;
        if self.state > 100 {
            self.state = 1;
        }

        self.num_rolled += 1;

        self.state
    }
}

#[derive(Hash, PartialEq, Eq, Clone)]
enum Player {
    P1,
    P2
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct GameState {
    p1_pos:i32,
    p2_pos:i32,
    p1_score:i32,
    p2_score:i32,
    to_go:Player
}

impl GameState {
    fn get_won(&self) -> Option<Player> {
        if self.p1_score >= 21 {
            Some(Player::P1)
        } else if self.p2_score >= 21 {
            Some(Player::P2)
        } else {
            None
        }
    }

    fn next_for(&self, roll:i32) -> Self {
        let mut next = self.clone();

        match next.to_go {
            Player::P1 => {
                next.p1_pos += roll;
                while next.p1_pos > 10 {
                    next.p1_pos -= 10;
                }
                next.p1_score += next.p1_pos;
                next.to_go = Player::P2;
            },
            Player::P2 => {
                next.p2_pos += roll;
                while next.p2_pos > 10 {
                    next.p2_pos -= 10;
                }
                next.p2_score += next.p2_pos;
                next.to_go = Player::P1;
            }
        }

        next
    }
}

pub fn run() {
    run_part_01();
    run_part_02();
}

fn run_part_01() {
    let mut p1_pos = 7;
    let mut p2_pos = 10;

    let mut p1_score = 0;
    let mut p2_score = 0;

    let mut roller = DeterministicRoller::new();

    while p1_score < 1000 && p2_score < 1000 {
        for _ in 0..3 {
            p1_pos += roller.roll();
        }
        while p1_pos > 10 {
            p1_pos -= 10;
        }

        p1_score += p1_pos;

        if p1_score >= 1000 {
            break;
        }

        for _ in 0..3 {
            p2_pos += roller.roll();
        }
        while p2_pos > 10 {
            p2_pos -= 10;
        }

        p2_score += p2_pos;
    }

    let low_score = i32::min(p1_score, p2_score);

    let product = low_score * roller.num_rolled;

    println!("The loser's score is {}, and the dice was rolled {} times", low_score, roller.num_rolled);
    println!("The product is {}", product);
}

fn run_part_02() {
    let orig_state = GameState { p1_pos: 7, p2_pos: 10, p1_score: 0, p2_score: 0, to_go: Player::P1 };

    let mut current_states:HashMap<GameState, u64> = HashMap::new();
    let mut state_list:HashMap<GameState, bool> = HashMap::new();

    let mut p1_wins:HashMap<GameState, u64> = HashMap::new();
    let mut p2_wins:HashMap<GameState, u64> = HashMap::new();

    state_list.insert(orig_state.clone(), true);
    current_states.insert(orig_state, 1);

    let mut moved = true;
    let mut p = 0;
    while moved == true {
        moved = false;

        let mut to_add:Vec<(GameState, u64)> = Vec::new();

        for (state, updated) in state_list.iter_mut() {
            if *updated {
                match current_states.get_mut(state) {
                    Some(n) if *n > 0 => {
                        for roll1 in 1..=3 {
                            for roll2 in 1..=3 {
                                for roll3 in 1..=3 {
                                    to_add.push((state.next_for(roll1 + roll2 + roll3), *n));
                                }
                            }
                        }
    
                        *n = 0;
    
                        moved = true;
                    },
                    _ => ()
                }

                *updated = false;
            }
        }

        for (s, n) in to_add.into_iter() {
            match s.get_won() {
                Some(Player::P1) => {
                    *p1_wins.entry(s).or_insert(0) += n;
                },
                Some(Player::P2) => {
                    *p2_wins.entry(s).or_insert(0) += n;
                },
                None => {
                    *state_list.entry(s.clone()).or_default() = true;
                    *current_states.entry(s).or_insert(0) += n;
                }
            }
        }

        p += 1;
    }
    println!("{} Iterations", p);

    let p1_score:u64 = p1_wins.values().sum();
    let p2_score:u64 = p2_wins.values().sum();

    println!("Player 1 wins in {} universes while Player 2 wins in {} universes", p1_score, p2_score);
}