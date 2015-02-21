#![feature(core)]
fn main() {
    println!("Hello, world!");

    let v = vec!["match this", "1"];

    match v.as_slice() {
        ["match this", second] => println!("the second element is {}", second),
        _ => {},
    }
}
