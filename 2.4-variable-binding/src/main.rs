fn main(){
    let mut x: i32;
    let z: i32 = 5i32;
    x = 632;
    println!("Hello, world = {0}", x * z);

    let (a,b) = (1i32, 1.0f64 / 3.0f64 );
    println!("a {a} {b} {x}", a=a, b=b, x=x);

    if x == 732 {
        println!("x == 6 integer");
    }

    let y: i32 = if x == 632 { 1i32 } else { 2i32 };
    println!("y == x{0:X}", y);

    let q = 5us;
    println!("q {:08}", q);
}
