use std::io;
use std::rand;

fn my_random(min: uint, max: uint) -> uint {
    (rand::random::<uint>() %  max ) +  min
}

fn main() {
    println!("Guess the number!");

    let secret = my_random(1u, 200u);
    //println!("The secret number is: {}", secret);
    println!("Please input your guess.");

    loop {
        let input = io::stdin().read_line()
            .ok()
            .expect("Failed to read line");

        let input_num  = match from_str::<uint>(input.as_slice().trim()) {
            Some(num) => num,
            None => {
                println!("Please input a number!");
                continue;
            },
        };

        println!("You guessed: {}", input_num);
        match cmp(input_num, secret) {
            Less => println!("Too Small!"),
            Greater => println!("Too big!"),
            Equal => {
                println!("You win!");
                return;
            },
        }
    }
}

fn cmp(a: uint, b: uint) -> Ordering {
    if a < b { Less }
    else if a > b { Greater }
    else { Equal }
}
