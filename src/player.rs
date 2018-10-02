extern crate rand;

use self::rand::Rng;

use board::Board;

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
