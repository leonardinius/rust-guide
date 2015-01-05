enum OptionalInt {
    Value(int),
    Missing
}

#[warn(dead_code)]
fn main() {
    println!("Hello, world!");

    let x = OptionalInt::Value(15i);

    match x {
        OptionalInt::Value(i) if i > 5 => println!("Got an int bigger than five!"),
        OptionalInt::Value(..) => println!("Got an int!"),
        OptionalInt::Missing => println!("No such luck"),
    };
}
