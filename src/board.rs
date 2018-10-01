extern crate rand;
use std::fmt;

#[derive(Debug)]
pub struct Board {
    name: String,
    board: [char; 9],
    move_count: i8,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\
         {} | {} | {}\n\
        ---------\n\
         {} | {} | {}\n\
        ---------\n\
         {} | {} | {}", self.board[0], self.board[1], self.board[2],
                        self.board[3], self.board[4], self.board[5],
                        self.board[6], self.board[7], self.board[8])
    }
}

impl Board {
    const WINNING_MOVES: [(usize, usize, usize); 8] = [
        (0, 1, 2),
        (3, 4, 5),
        (6, 7, 8),
        (0, 3, 6),
        (1, 4, 7),
        (2, 5, 8),
        (0, 4, 8),
        (2, 4, 6),
    ];

    const SYMBOLS: [char; 2] = ['X', 'O'];

    pub fn new(name: String) -> Board {
        Board {
            name: name,
            board: [' '; 9],
            move_count: 0,
        }
    }

    /*
    pub fn clear(&mut self) {
        self.board = [' '; 9];
        self.move_count = 0;
    }
    */

    pub fn next_move(&self) -> char {
        let idx = self.move_count % 2;
        Board::SYMBOLS[idx as usize]
    }

    pub fn make_move(&mut self, location: usize) -> Result<(), &str> {
        if self.board[location] != ' ' {
            return Err("Board location already filled");
        }

        self.board[location] = self.next_move();
        self.move_count += 1;
        Ok(())
    }

    pub fn openings(&self) -> Vec<usize> {
        let mut result = Vec::new();
        for i in 0..9 {
            if self.board[i] == ' ' {
                result.push(i as usize);
            }
        }

        result
    }

    pub fn is_draw(&self) -> bool {
        self.winner().is_none() && self.move_count == 9
    }

    pub fn winner(&self) -> Option<char> {
        for winning_move in Board::WINNING_MOVES.iter() {
            let winner = self.board[winning_move.0];

            if winner != ' ' &&
               winner == self.board[winning_move.1] &&
               winner == self.board[winning_move.2] {
                return Some(winner)
            }
        }
        None
    }
}


use board::rand::Rng;

#[derive(Debug)]
pub struct AutoPlayer {
    weights: [f64; 9],
    pub moves: Vec<usize>,
}

impl AutoPlayer {
    pub fn new() -> AutoPlayer {
        AutoPlayer {
            weights: [0.5; 9],
            moves: Vec::new(),
        }
    }

    pub fn random() -> AutoPlayer {
        let weights: [f64; 9] = rand::thread_rng().gen();
        AutoPlayer {
            weights: weights,
            moves: Vec::new(),
        }
    }

    pub fn weighted(weights: [f64; 9]) -> AutoPlayer {
        AutoPlayer {
            weights: weights,
            moves: Vec::new(),
        }
    }

    pub fn make_move(&mut self, board: &mut Board) {
        let openings = board.openings();

        let mut max_weight = self.weights[openings[0]];
        let mut choice = 0;
        for i in openings {
            if !self.moves.contains(&i) && self.weights[i] >= max_weight {
                max_weight = self.weights[i];
                choice = i;
            }
        }

        board.make_move(choice).unwrap();
        self.moves.push(choice);
    }

    pub fn finalize(&mut self, end_state: i8) {
        for m in &self.moves {
            if end_state == 1 {
                let diff = (1.0 - self.weights[*m]) / 2.0;
                self.weights[*m] += diff;
            }
            else if end_state == -1 {
                let diff = self.weights[*m] / 2.0;
                self.weights[*m] -= diff;
            }
        }

        self.moves.clear();
    }
}
