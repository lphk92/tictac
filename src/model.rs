use std;

#[derive(Debug)]
pub struct Board {
    name: String,
    board: [char; 9],
    move_count: i8,
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
