enum OptionalInt {
    Value(i32),
    Missing
}

#[warn(dead_code)]
fn main() {
    println!("Hello, world!");

    let x = OptionalInt::Value(15i32);

    match x {
        OptionalInt::Value(i) if i > 5 => println!("Got an i32 bigger than five!"),
        OptionalInt::Value(..) => println!("Got an i32!"),
        OptionalInt::Missing => println!("No such luck"),
    };

    let y = OptionalInt::Missing;

    match y {
        OptionalInt::Value(i) if i > 5 => println!("Got an i32 bigger than five!"),
        OptionalInt::Value(..) => println!("Got an i32!"),
        OptionalInt::Missing => println!("No such luck"),
    };
}
