fn main() {
    println!("Hello, world!");

    let x = 3i32;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 | 4 => println!("three or four"),
        _ => println!("anything"),
    }

    match x {
        1 ... 5 => println!("one through five"),
        _ => println!("anything"),
    }

    match x {
        e @ 1 ... 5 => println!("got a range element {}", e),
        _ => println!("anything"),
    }

    {
        let x = &5i32;
        match x {
            &val => println!("Got a value: {}", val),
        }
    }

    {
        let x = 5i32;
        match x {
            ref r => println!("Got a reference to: {}", r),
        }
    }

    {
        let mut x = 5i32;
        match x {
            ref mut mr => println!("Got a mutable reference to: {}", mr),
        }
    }
}
