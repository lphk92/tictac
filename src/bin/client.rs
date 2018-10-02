use std::io;
use std::io::prelude::*;
use std::net::TcpStream;


fn prompt(msg: &String) -> String {
    let mut input = String::new();
    print!("{}", msg);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    String::from(input.trim())
}

fn main() {
    let host = "127.0.0.1";
    let port = 8074;

    println!("Connecting to server on {}:{}", host, port);
    let mut stream = TcpStream::connect(format!("{}:{}", host, port)).unwrap();

    let mut message = prompt(&String::from("Enter a message: "));

    while message != String::from("quit") {
        println!("Message: <-- {} -->", message);
        stream.write(message.as_bytes()).unwrap();
        stream.flush().unwrap();
        message = prompt(&String::from("Enter a message: "));
    }
}
