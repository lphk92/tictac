use std::io;
use std::thread;
use std::sync::mpsc;
extern crate time;

mod board;

fn prompt(msg: &String) -> String {
    let mut input = String::new();
    println!("{}", msg);
    io::stdin().read_line(&mut input).unwrap();
    String::from(input.trim())
}

fn play_one(computer: &mut board::AutoPlayer) -> (i8, Vec<usize>) {
    let mut board = board::Board::new(String::from("Test Board"));
    let mut opponent = board::AutoPlayer::random();

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

    computer.finalize(computer_won);
    (computer_won, computer.moves.to_owned())
}

fn generate_weights(results: &Vec<(i8, Vec<usize>)>) -> [f64; 9] {
    let mut weights = [0.5; 9];
    // TODO: Generate better weights
    for (state, moves) in results.iter() {

    }
    weights
}

fn train_sync(n_times: i32) -> [f64; 9] {
    let mut results = Vec::<(i8, Vec<usize>)>::new();

    for _ in 0..n_times {
        let mut computer = board::AutoPlayer::new();
        let result = play_one(&mut computer);
        results.push(result);
    }

    generate_weights(&results)
}

fn train_channel(n_workers: i32, n_times: i32) -> [f64; 9] {
    let mut results = Vec::<(i8, Vec<usize>)>::new();

    let (tx, rx) = mpsc::channel();

    let times_per_worker = n_times / n_workers;

    for _ in 0..n_workers {
        let curr_tx = mpsc::Sender::clone(&tx);
        thread::spawn(move || {
            let mut computer = board::AutoPlayer::new();

            for _ in 0..times_per_worker {
                let result = play_one(&mut computer);
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

    generate_weights(&results)
}

fn main() {
    let mut board = board::Board::new(String::from("Test Board"));

    let mut computer = board::AutoPlayer::new();

    // Training Loop
    let n_workers = 4;
    let n_times = 100000;
    println!("Training computer {} times.", n_times);

    {
        let start = time::precise_time_s();
        let weights = train_sync(n_times);
        let end = time::precise_time_s();
        println!("Training time (syncronous): {:.4}s", end - start);
    }

    {
        let start = time::precise_time_s();
        let weights = train_channel(n_workers, n_times);
        let end = time::precise_time_s();
        println!("Training time (channel): {:.4}s", end - start);
    }

    /*
    for _ in 0..n_times {
        let mut opponent = board::AutoPlayer::random();
        while board.winner().is_none() && !board.is_draw(){
            if board.next_move() == 'X' {
                opponent.make_move(&mut board);
            }
            else {
                computer.make_move(&mut board);
            }
        }
        computer.finalize(match board.winner() {
            Some(symbol) => symbol == 'O',
            None => false
        });
        board.clear();
    }
    */


    // Game loop
    while board.winner().is_none() && !board.is_draw(){
        if board.next_move() == 'X' {
            println!("\n{}", board);
            let input = prompt(&format!("Player {}, make your move: ", board.next_move()));

            let location = input.parse::<i8>().unwrap();
            match board.make_move(location as usize) {
                Ok(i) => i,
                Err(e) => println!("Player Error: {}", e)
            };
        }
        else {
            computer.make_move(&mut board);
            println!("{:?}", computer);
        }
    }

    // Print winner
    println!("\n\nWinner: {}", match board.winner() {
        Some(symbol) => symbol.to_string(),
        None => String::from("Draw")
    });
    println!("{}", board);
    println!("{:?}", computer);
}
