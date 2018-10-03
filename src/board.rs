use std::fmt;
use std::char;

extern crate ansi_term;
use self::ansi_term::Colour::Green;

#[derive(Debug)]
pub struct Board {
    name: String,
    board: [char; 9],
    move_count: i8,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut arr: Vec<String> = vec![String::new(); 9];
        for i in 0..self.board.len() {
            if self.board[i] != ' ' {
                arr[i] = char::to_string(&self.board[i]);
            }
            else {
                arr[i] = match char::from_digit((i + 1) as u32, 10) {
                    Some(c) => Green.paint(char::to_string(&c)).to_string(),
                    None => String::new()
                };
            }
        }

        write!(f, "\
         {} | {} | {}\n\
        ---------\n\
         {} | {} | {}\n\
        ---------\n\
         {} | {} | {}", arr[0], arr[1], arr[2],
                        arr[3], arr[4], arr[5],
                        arr[6], arr[7], arr[8])
    }
}

impl Board {
    pub const WINNING_MOVES: [[usize; 3]; 8] = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];

    pub const SYMBOLS: [char; 2] = ['X', 'O'];

    pub fn new(name: String) -> Board {
        Board {
            name: name,
            board: [' '; 9],
            move_count: 0,
        }
    }

    pub fn get_board(&self) -> &[char; 9] {
        &self.board
    }

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
            let winner = self.board[winning_move[0]];

            if winner != ' ' &&
               winner == self.board[winning_move[1]] &&
               winner == self.board[winning_move[2]] {
                return Some(winner)
            }
        }
        None
    }
}
