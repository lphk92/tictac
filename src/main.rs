mod model;

fn main() {
    println!("Hello, world!");

    let mut board = model::Board::new(String::from("Test Board"));

    println!("{:?}", board);
    println!("Winner: {}", match board.winner() {
        Some(symbol) => symbol.to_string(),
        None => String::from("None")
    });

    println!("Next Move: {}", board.next_move());
    board.make_move(0);
    println!("{:?}", board);
    println!("Next Move: {}", board.next_move());
}
