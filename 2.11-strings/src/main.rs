#![feature(core)]
fn takes_slice(slice: &str){
    println!("Got: {}", slice);
}

fn compare(string : String){
    if string.as_slice() == "Hello" {
        println!("yes");
    }
    else {
        println!("no");
    }
}

fn _11_1(){
    println!("guide 11-1");

    let mut s = "Hello".to_string();
    println!("{}", s);

    s.push_str(", world!");
    println!("{}", s);

    takes_slice(s.as_slice());

    compare("hello".to_string());
    compare("Hello".to_string());
}

fn main(){
    println!("guide 11");
    _11_1();
}
