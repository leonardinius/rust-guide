#![feature(core)]
fn _10_1(){
    println!("guide 10-1");

    for x in range (0i32, 10i32){
        println!("{}", x);
    }
}

fn _10_2(){
    println!("guide 10-2");

    let mut x = 5u64;
    let mut done = false;

    while !done {
        x += x -3;
        println!("{}", x);

        if x % 5 == 0 { done = true; }
    }
}

fn _10_3(){
    println!("guide 10-3");

    let mut x = 5u64;
    loop {
        x += x - 3;
        if x % 2 == 0 { continue; }
        println!("{}", x);
        if x % 9 == 0 { break; }
        x += 1;
    }
}

fn main(){
    println!("guide-10");
    _10_1();
    _10_2();
    _10_3();
}
