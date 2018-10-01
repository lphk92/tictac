use std::io;

mod model;

fn prompt(msg: &String) -> String {
    let mut input = String::new();
    println!("{}", msg);
    io::stdin().read_line(&mut input);
    String::from(input.trim())
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();

    let mut board = model::Board::new(String::from("Test Board"));

    println!("Winner: {}", match board.winner() {
        Some(symbol) => symbol.to_string(),
        None => String::from("None")
    });

    /*
    println!("Next Move: {}", board.next_move());
    board.make_move(0).unwrap();
    board.make_move(4).unwrap();
    board.make_move(1).unwrap();
    board.make_move(3).unwrap();
    board.make_move(2).unwrap();
    println!("{}", board);
    println!("Next Move: {}", board.next_move());
    */

    while board.winner().is_none() && !board.is_draw(){
        println!("{}", board);
        let input = prompt(&format!("Player {}, make your move: ", board.next_move()));

        let location = input.parse::<i8>().unwrap();
        board.make_move(location as usize);
    }

    println!("\n\nWinner: {}", match board.winner() {
        Some(symbol) => symbol.to_string(),
        None => String::from("None")
    });
    println!("{}", board);
}
