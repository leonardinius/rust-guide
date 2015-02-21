fn add_one(x: &i32) -> i32 {
    *x + 1
}

fn main(){
    let x = Box::new(5i32);

    println!("{}", add_one(&*x));
}
