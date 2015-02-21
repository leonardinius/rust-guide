fn main () {
    let mut x = 5i32;

    add_one(&mut x);
    println!("{:p} -> {}", &x, x);

    add_one(&mut x);
    println!("{:p} -> {}", &x, x);
}

fn add_one(num: &mut i32){
  *num += 1;
}
