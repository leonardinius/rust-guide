fn main() {
    println!("Hello, world!");

    let square = |&: x:i32| -> i32 { x * x };
    fn twice<F: Fn(i32) -> i32> (x: i32, f: F) -> i32 { f(x) + f(x) }
    fn square_fn (x: i32) -> i32 { x * x };

    let x = 10;
    println!("twice({}) -> {}", x, twice(x, square));
    println!("twice({}) -> {}", x, twice(x, square_fn));
}
