enum OptionalInt {
    Value(int),
    Missing,
}

fn _9_1(){
    println!("guide 9-1");
    let x = OptionalInt::Value(5);
    let y = OptionalInt::Missing;

    match x {
        OptionalInt::Value(n) => println!("x is {}", n),
        OptionalInt::Missing  => println!("x is missing!"),
    }

    match y {
        OptionalInt::Value(n) => println!("y is {}", n),
        OptionalInt::Missing  => println!("y is missing!"),
    }
}

fn cmp(a: int, b: int) -> Ordering {
    if a < b { Less }
    else if a > b { Greater }
    else { Equal }
}

fn _9_2(){
    println!("guide 9-2");
    let x = 5i;
    let y = 10i;

    println!("{}", match cmp(x, y) {
        Less    => "less",
        Greater => "greater",
        Equal   => "equal",
    });
}

fn main(){
    println!("guide-9");
    _9_1();
    _9_2();
}
