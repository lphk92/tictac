extern crate rand;
use self::rand::Rng;

use board::Board;
use utils::print_vec;

#[derive(Debug)]
pub struct AutoPlayer {
    weights: [f64; 9],
    pub moves: Vec<usize>,
    pub debug: bool,
}

impl AutoPlayer {
    pub fn random() -> AutoPlayer {
        let weights: [f64; 9] = rand::thread_rng().gen();
        AutoPlayer {
            weights: weights,
            moves: Vec::new(),
            debug: false,
        }
    }

    pub fn weighted(weights: [f64; 9]) -> AutoPlayer {
        AutoPlayer {
            weights: weights,
            moves: Vec::new(),
            debug: false,
        }
    }

    fn identify_wins(board: &Board, symbol: char) -> Vec<usize> {
        let mut wins: Vec<usize> = Vec::new();

        for state in Board::WINNING_MOVES.iter() {
            let mut winning_spot = 0;
            let mut symbol_count = 0;

            let board_state = board.get_board();

            for location in state {
                if board_state[*location] == symbol {
                    symbol_count += 1;
                }
                else if board_state[*location] == ' ' {
                    winning_spot = *location;
                }
                else {
                    // If it's not the symbol or empty, there is no win
                    symbol_count = 0;
                    break;
                }
            }

            if symbol_count == 2 {
                wins.push(winning_spot);
            }
        }
        wins
    }

    pub fn make_move(&mut self, board: &mut Board) {
        let openings = board.openings();

        let self_wins = AutoPlayer::identify_wins(board, board.next_move());
        let opponent_wins = AutoPlayer::identify_wins(board, if board.next_move() == 'O' { 'X' } else {'O'});

        if self.debug {
            print!("Self Wins: "); print_vec(&self_wins);
            print!("Opponent Wins: "); print_vec(&opponent_wins);
        }

        let mut choice = 0;
        if self_wins.len() > 0 {
            choice = self_wins[0];
            if self.debug { println!("Here I would try to take self win {}", choice) }
        }
        else if opponent_wins.len() > 0 {
            choice = opponent_wins[0];
            if self.debug { println!("Here I would try to take opponent win {}", choice) }
        }
        else {
            let mut max_weight = self.weights[openings[0]];
            for i in openings {
                if !self.moves.contains(&i) && self.weights[i] >= max_weight {
                    max_weight = self.weights[i];
                    choice = i;
                }
            }
        }

        board.make_move(choice).unwrap();
        self.moves.push(choice);
    }
}
