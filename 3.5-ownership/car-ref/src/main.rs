#![feature(core)]
use std::rc::Rc;

#[derive(Debug)]
struct Car {
    name : String,
}

#[derive(Debug)]
struct Wheel<'a> {
    size : i32,
    owner : &'a Car,
}

fn main () {
    let car = Rc::new(Car {name: "DeLorean".to_string() });

    let mut v : Vec<Wheel> = Vec::new();

    for _ in range(0, 4) {
        let c = Wheel { size : 360, owner : &*car };
        v.push(c);
    }

    println!("{:?}", v);
}
