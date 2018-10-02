use std::thread;
use std::io;
use std::io::Read;
use std::io::Write;
use std::net::{TcpListener, TcpStream};

extern crate tictac;
use tictac::player::AutoPlayer;

fn handle_connection(mut stream: TcpStream) {
    //let mut buffer = [0; 512];
    //stream.read(&mut buffer).unwrap();
    println!("Connection opened!");

    loop {
        let mut buffer = String::new();
        stream.read_to_string(&mut buffer).unwrap();

        if buffer.len() == 0 {
            break;
        }

        println!("Received message: {}", buffer);
        io::stdout().flush().unwrap();
    }

    println!("Connection dropped!");
}

fn main() {
    let host = "127.0.0.1";
    let port = 8074;

    println!("Listening on {}:{}", host, port);
    let listener = TcpListener::bind(format!("{}:{}", host, port)).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}
