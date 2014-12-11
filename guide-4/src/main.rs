fn main(){
    let mut x = 5i;
    let z: int = 5i;
    x = 6i;
    println!("Hello, world = {:d}", x * z);

    let (a,b) = (1i, 1.0f64 / 3.0f64 );
    println!("a {:d} b {:f} {}", a, b, x);

    if x == 7i {
        println!("x == 6 integer");
    }

    let y: int = if x == 6i { 1i } else { 2i };
    println!("y == {}", y);
}
