use std::io;

mod board;

fn prompt(msg: &String) -> String {
    let mut input = String::new();
    println!("{}", msg);
    io::stdin().read_line(&mut input).unwrap();
    String::from(input.trim())
}

fn main() {
    let mut board = board::Board::new(String::from("Test Board"));

    let mut computer = board::AutoPlayer::new();

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

    computer.finalize(match board.winner() {
        Some(symbol) => symbol == 'O',
        None => false
    });

    // Print winner
    println!("\n\nWinner: {}", match board.winner() {
        Some(symbol) => symbol.to_string(),
        None => String::from("Draw")
    });
    println!("{}", board);
    println!("{:?}", computer);
}
