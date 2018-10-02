use std::env;
use std::io;

extern crate time;

mod board;
mod player;
mod training;

use player::AutoPlayer;

fn prompt(msg: &String) -> String {
    let mut input = String::new();
    println!("{}", msg);
    io::stdin().read_line(&mut input).unwrap();
    String::from(input.trim())
}

fn print_vec(v: &Vec<usize>) {
    println!("Printing vector of size {}", v.len());
    for item in v {
        println!("   {}", item);
    }
}

fn print_floats(v: &Vec<f64>) {
    println!("Printing vector of size {}", v.len());
    for item in v {
        println!("   {}", item);
    }
}


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

    training::train_sync(n_times);
    training::train_channel(n_workers, n_times);
    let weights = training::train_mutex(n_workers, n_times);

    println!("{} {} {} {} {} {} {} {} {}",
             weights[0],
             weights[1],
             weights[2],
             weights[3],
             weights[4],
             weights[5],
             weights[6],
             weights[7],
             weights[8]);

    let mut board = board::Board::new(String::from("Test Board"));
    let mut computer = AutoPlayer::weighted(weights);

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
