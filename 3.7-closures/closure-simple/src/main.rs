fn main() {
    println!("Hello, world!");

    let add_one = |&: x| {1 + x };
    println!("The sum of 5 plus is {}", add_one(5));

    let x = &mut 5;
    let full_closure =|&: | { println!("full closure, x is {}", x); };
    //*x = 8i32;
    full_closure();
    println!("x={}", x);
}
