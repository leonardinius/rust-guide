fn main () {
    let mut x = Box::new(5i32);

    x = add_one(x);
    println!("{:p} -> {}", &x, x);

    x = add_one(x);
    println!("{:p} -> {}", &x, x);
}

fn add_one(mut num: Box<i32>) -> Box<i32>{
  *num += 1;

  num
}
