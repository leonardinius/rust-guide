use std::cmp::Ordering;

fn _8_1(){
    let x1 = ( 1i32, "hello");
    println!("tuple x1 = {:?}", x1);

    let x2: (i32, &str) = (2, "my world");
    println!("x2 = {:?}", x2);

    let (x, y, z) = (1i32, 2i32, 3i32);
    println!("x = {}, y = {}, z = {}", x, y, z);

    let mut a;
    let b = (2i32, 3i32);
    a = b;
    println!("a = b = {:?}", a);

    let q = (1i32, 2i32, 3i32);
    let w = (2i32, 2i32, 4i32);

    if q == w {
            println!("yes");
    } else {
            println!("no");
    }
}

struct Point {
    x: i32,
    y: i32,
}

fn _8_2() {
    let origin = Point { x: 0i32, y: 0i32 };
    println!("The origin is at ({}, {})", origin.x, origin.y);

    let mut o2 = Point { x: 1i32, y: 2i32 };
    o2.x = 10i32;
    println!("The origin is at ({}, {})", o2.x, o2.y);
}

#[derive(Debug)]
struct Inches(i32);

fn _8_3(){
    let length = Inches(10);

    let Inches(a) = length;

    println!("length {:?} is {} {:X} inches", length, a, a);
}

fn cmp(a: i32, b: i32) -> Ordering {
    if a < b { Ordering::Less }
    else if a > b { Ordering::Greater }
    else { Ordering::Equal }
}

fn _8_4() {
    let x = 5i32;
    let y = 10i32;

    let ordering = cmp(x, y);

    if ordering == Ordering::Less {
        println!("less");
    } else if ordering == Ordering::Greater {
        println!("greater");
    } else if ordering == Ordering::Equal {
        println!("equal");
    }
}

fn main(){
  _8_1();
  _8_2();
  _8_3();
  _8_4();
}
