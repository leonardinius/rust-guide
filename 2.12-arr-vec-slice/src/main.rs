fn _12_1(){
    println!("guide 12-1");

    let a = [1i, 2i, 3i];
    let mut m = [2i, 3i, 4i];

    let b = [0i, ..20]; // shorthand for array of 20 elements all initialized to 0
    println!("{}", b);
    m = [5i, 6i, 7i];
    println!("{}", m);

    for i in m.iter() {
        println!("elem {}", i);
    }

    let names = ["Emilija", "Anzelika"];
    println!("{} -> {}", names[1], names[0]);
}

fn _12_2(){
    println!("guide 12-2");

    let mut v = vec![1i, 2, 3];
    v.push(4);
    println!("{}, len is {}", v, v.len());
}

fn _12_3(){
    println!("guide 12-3");

    let mut a = vec![0i, 1, 2, 3, 4];
    let middle = a.as_mut_slice(); 

    middle[0] = 10i;

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
