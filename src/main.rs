use std::io;

mod model;

fn prompt(msg: &String) -> String {
    let mut input = String::new();
    println!("{}", msg);
    io::stdin().read_line(&mut input).unwrap();
    String::from(input.trim())
}

fn main() {
    let mut board = model::Board::new(String::from("Test Board"));

    // Game loop
    while board.winner().is_none() && !board.is_draw(){
        println!("\n{}", board);
        let input = prompt(&format!("Player {}, make your move: ", board.next_move()));

        let location = input.parse::<i8>().unwrap();
        match board.make_move(location as usize) {
            Ok(i) => i,
            Err(e) => println!("{}", e)
        };
    }

    // Print winner
    println!("\n\nWinner: {}", match board.winner() {
        Some(symbol) => symbol.to_string(),
        None => String::from("Draw")
    });
    println!("{}", board);
}
