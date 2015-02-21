#![feature(rand)]
#![feature(old_io)]
#![allow(deprecated)]
use std::old_io as io;
use std::rand;
use std::cmp::Ordering;

fn my_random(min: usize, max: usize) -> usize {
    (rand::random::<usize>() %  max ) +  min
}

fn main() {
    println!("Guess the number!");

    let secret = my_random(1us, 200us);
    //println!("The secret number is: {}", secret);
    println!("Please input your guess, between 1 and 200");

    loop {
        let input = io::stdin().read_line()
            .ok()
            .expect("Failed to read line");

        let input_trim = input.trim();
        let input_parse = input_trim.parse::<usize>();
        let input_num  = match input_parse {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number!");
                continue;
            },
        };

        println!("You guessed: {}", input_num);
        match cmp(input_num, secret) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                return;
            },
        }
    }
}

fn cmp(a: usize, b: usize) -> Ordering {
    if a < b { Ordering::Less }
    else if a > b { Ordering::Greater }
    else { Ordering::Equal }
}
