#![feature(old_io)]
use std::old_io as io;

fn main() {
    println!("Type something!");

    let input = io::stdin()
        .read_line()
        .ok()
        .expect("Failed to read line");

    println!("{}", input);
}

