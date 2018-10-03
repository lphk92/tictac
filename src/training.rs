use std::thread;
use std::sync::mpsc;
use std::sync::{Mutex, Arc};

extern crate time;

use board;
use player::AutoPlayer;

fn play_one() -> (i8, Vec<usize>) {
    let mut board = board::Board::new(String::from("Test Board"));
    let mut computer = AutoPlayer::random();
    let mut opponent = AutoPlayer::random();

    while board.winner().is_none() && !board.is_draw(){
        if board.next_move() == 'X' {
            opponent.make_move(&mut board);
        }
        else {
            computer.make_move(&mut board);
        }
    }

    let computer_won = match board.winner() {
        Some(symbol) => if symbol == 'O' { 1 } else { -1 },
        None => 0
    };

    (computer_won, computer.moves.to_owned())
}

fn generate_weights(results: &Vec<(i8, Vec<usize>)>) -> [f64; 9] {
    let mut win_count = [0 as i64; 9];
    // TODO: Generate better weights
    for (state, moves) in results {
        for m in moves {
            win_count[*m] += *state as i64;
        }
    }

    let n_results = results.len();
    let mut weights = [0.0; 9];
    for i in 0..win_count.len() {
        weights[i] = win_count[i] as f64 / n_results as f64;
    }

    weights
}

pub fn train_sync(n_times: i32) -> [f64; 9] {
    let start = time::precise_time_s();

    let mut results = Vec::<(i8, Vec<usize>)>::new();

    for _ in 0..n_times {
        let result = play_one();
        results.push(result);
    }

    let end = time::precise_time_s();
    println!("Training time (sync): {:.4}s", end - start);

    generate_weights(&results)
}

pub fn train_channel(n_workers: i32, n_times: i32) -> [f64; 9] {
    let start = time::precise_time_s();

    let (tx, rx) = mpsc::channel();
    let times_per_worker = n_times / n_workers;
    let mut results = Vec::<(i8, Vec<usize>)>::new();

    for _ in 0..n_workers {
        let curr_tx = mpsc::Sender::clone(&tx);
        thread::spawn(move || {
            for _ in 0..times_per_worker {
                let result = play_one();
                curr_tx.send(result).unwrap();
            }
        });
    }

    // Since we make a clone on each for loop iteration,
    // we need to delete the original so it doesn't hold up the receiver
    drop(tx);

    // Listen here and plush things onto results
    for result in rx {
        results.push(result);
    }

    let end = time::precise_time_s();
    println!("Training time (channel): {:.4}s", end - start);

    generate_weights(&results)
}

pub fn train_mutex(n_workers: i32, n_times: i32) -> [f64; 9] {
    let start = time::precise_time_s();

    let times_per_worker = n_times / n_workers;
    let mutex = Arc::new(Mutex::new(Vec::<(i8, Vec<usize>)>::new()));
    let mut handles = vec![];

    for _ in 0..n_workers {
        let mutex = Arc::clone(&mutex);
        let handle = thread::spawn(move || {
            for _ in 0..times_per_worker {
                let result = play_one();
                let mut results = mutex.lock().unwrap();

                results.push(result);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let results = mutex.lock().unwrap();

    let end = time::precise_time_s();
    println!("Training time (mutex): {:.4}s", end - start);

    generate_weights(&results)
}

