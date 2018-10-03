use std::io;
use std::io::Write;
use std::fmt;


pub fn prompt(msg: &String) -> String {
    let mut input = String::new();
    print!("{}", msg);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    String::from(input.trim())
}

pub fn print_vec<T: fmt::Display>(v: &Vec<T>) {
    print!("[");
    for item in v {
        print!("{}, ", item);
    }
    print!("]\n");
    io::stdout().flush().unwrap();
}
