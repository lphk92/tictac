mod model;

fn main() {
    println!("Hello, world!");

    let board = model::Board::new(String::from("Test Board"));

    println!("{:?}", board);
    println!("Winner: {}", board.winner());
}
