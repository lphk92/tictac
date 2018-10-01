#[allow(dead_code)]
#[derive(Debug)]
pub struct Board {
    name: String,
    board: [char; 9]
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

    pub fn new(name: String) -> Board {
        Board {
            name: name,
            board: [' '; 9]
        }
    }

    pub fn winner(&self) -> char {
        for winning_move in Board::WINNING_MOVES.iter() {
            let winner = self.board[winning_move.0];

            if winner == self.board[winning_move.1] &&
               winner == self.board[winning_move.2] {
                return winner
            }
        }
        ' '
    }
}
