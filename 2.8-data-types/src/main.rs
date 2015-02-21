fn _8_1(){
    let x1 = ( 1i, "hello");
    println!("tuple x1 = {}", x1);

    let x2: (int, &str) = (2, "my world");
    println!("x2 = {}", x2);

    let (x, y, z) = (1i, 2i, 3i);
    println!("x = {}, y = {}, z = {}", x, y, z);

    let mut a = (1i, 2i);
    let b = (2i, 3i);
    a = b;
    println!("a = b = {}", a);

    let q = (1i, 2i, 3i);
    let w = (2i, 2i, 4i);

    if q == w {
            println!("yes");
    } else {
            println!("no");
    }
}

struct Point {
    x: int,
    y: int,
}

fn _8_2() {
    let origin = Point { x: 0i, y: 0i };
    println!("The origin is at ({}, {})", origin.x, origin.y);

    let mut o2 = Point { x: 1i, y: 2i };
    o2.x = 10i;
    println!("The origin is at ({}, {})", o2.x, o2.y);
}

struct Inches(int);

fn _8_3(){
    let length = Inches(10);

    let Inches(a) = length;

    println!("length is {} {1:X} inches", a, a);
}

fn cmp(a: int, b: int) -> Ordering {
    if a < b { Less }
    else if a > b { Greater }
    else { Equal }
}

fn _8_4() {
    let x = 5i;
    let y = 10i;

    let ordering = cmp(x, y);

    if ordering == Less {
        println!("less");
    } else if ordering == Greater {
        println!("greater");
    } else if ordering == Equal {
        println!("equal");
    }
}

fn main(){
  _8_1();
  _8_2();
  _8_3();
  _8_4();
}
