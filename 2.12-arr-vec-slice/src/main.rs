fn _12_1(){
    println!("guide 12-1");

    let a = [1i32, 2i32, 3i32];
    let mut m = [2i32, 3i32, 4i32];
    if false {
      println!("{:?} {:?}", a, m);
    }

    let b = [0i32; 20]; // shorthand for array of 20 elements all initialized to 0
    println!("{:?}", b);
    m = [5i32, 6i32, 7i32];
    println!("{:?}", m);

    for i in m.iter() {
        println!("elem {}", i);
    }

    let names = ["Emilija", "Anzelika"];
    println!("{} -> {}", names[1], names[0]);
}

fn _12_2(){
    println!("guide 12-2");

    let mut v = vec![1i32, 2, 3];
    v.push(4);
    println!("{:?}, len is {}", v, v.len());
}

fn _12_3(){
    println!("guide 12-3");

    let mut a = vec![0i32, 1, 2, 3, 4];
    let middle = a.as_mut_slice();

    middle[0] = 10i32;

    for e in middle.iter() {
        println!("{}", e);
    }

}

fn main(){
    println!("guide 12");
    _12_1();
    _12_2();
    _12_3();
}
