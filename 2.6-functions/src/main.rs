fn factorial(i : u64) -> u64 {
    match i {
      1 | 0 => 1,
      _ => i * factorial(i - 1),
    }
}

fn main(){
    let o: u64 = 5u64;
    let x = factorial(o);

    let y = format!("{:b}", x).len() as u64;
    println!("factorial({}) = {} ({:b} / {})", o, x, x, y);

    let q = 15u64;
    if y < q {
        println!("x < {} = {}", q, x);
    }

    let u = if x < 5u64 { 0i32 } else { 1i32 };
    println!("u = {}", u);

    let u1 = if x < 5u64 { 0i32; } else { 1i32; };
    println!("u1 = {:?}", u1);
}
