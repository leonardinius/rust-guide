use std::rc::Rc;

fn main() {
    println!("Hello, world!");
    
    let x = Rc::new(5i);
    let y = x.clone();

    println!("{} {}", *x, *y);
}
