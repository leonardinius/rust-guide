fn main() {
    let x = 4;

    match x {
        1 | 2  => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }


    match x {
        1...5 => println!("one through five"),
        _ => println!("anything"),
    }

    match x {
        e @ 1...5 => println!("one through five, {}", e),
        _ => println!("anything"),
    }
}
