use std::env;

extern crate rand;
use rand::{thread_rng, Rng};

extern crate tictac;
use tictac::board;
use tictac::training;
use tictac::player::AutoPlayer;
use tictac::utils::{prompt, print_vec};


fn main() {
    let args: Vec<String> = env::args().collect();

    // Training Loop
    let mut n_workers = 4;

    if args.len() > 1 {
        n_workers = match args[1].parse::<i32>() {
            Ok(i) => i,
            Err(_) => n_workers
        };
    }
    let n_times = 100000;
    println!("Training computer {} times with {} workers.", n_times, n_workers);

    //training::train_sync(n_times);
    //training::train_channel(n_workers, n_times);
    let weights = training::train_mutex(n_workers, n_times);

    print_vec::<f64>(&weights.to_vec());

    let mut board = board::Board::new(String::from("Test Board"));
    let mut computer = AutoPlayer::weighted(weights);
    //computer.debug = true;

    // Game loop
    let player_symbol = *(match thread_rng().choose(&board::Board::SYMBOLS) {
        Some(s) => s,
        None => &'X'
    });
    println!("Player is {}", player_symbol);

    while board.winner().is_none() && !board.is_draw(){
        if board.next_move() == player_symbol {
            println!("\n\n{}", board);
            let input = prompt(&format!("Player {}, make your move: ", board.next_move()));

            let location = input.parse::<i8>().unwrap() - 1;
            match board.make_move(location as usize) {
                Ok(i) => i,
                Err(e) => println!("Player Error: {}", e)
            };
        }
        else {
            computer.make_move(&mut board);
        }
    }

    // Print winner
    println!("\n\nWinner: {}", match board.winner() {
        Some(symbol) => symbol.to_string(),
        None => String::from("Draw")
    });
    println!("{}", board);

    println!("\nThanks for Playing!");
    //println!("{:?}", computer);
}
