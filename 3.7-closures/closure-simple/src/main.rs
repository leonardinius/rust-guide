fn main() {
    println!("Hello, world!");

    let add_one = |&: x| {1 + x };
    println!("The sum of 5 plus is {}", add_one(5));
}
