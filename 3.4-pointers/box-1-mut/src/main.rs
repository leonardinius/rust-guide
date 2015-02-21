fn add_one<'a>(x: &'a mut i32) -> i32 {
    *x += 1;
    *x
}

fn main(){
    let mut x = Box::new(5i32);

    println!("{}", add_one(&mut *x));
    println!("{}", add_one(&mut *x));
    println!("{}", add_one(&mut *x));
}
