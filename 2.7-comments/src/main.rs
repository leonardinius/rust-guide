//! A minimal implementation of SHA1 for rust.
//!
//! Example:
//!
//! ```rust
//! ```

/// `factorial` is  a function that returns a n! from n
///
/// # Arguments
///
/// * `n` - The non-negative number to calculate the factorial to
///
/// # Example
/// ```
/// // example
/// let n: u64 = 5u64;
/// let result: u64 = hello_world::factorial(n);
/// println!("{}! = {}", n, result); // pri32s "5! = 120"
/// ```
pub fn factorial(n : u64) -> u64 {
    match n {
      1 | 0 => 1,
      _ => n * factorial(n - 1),
    }
}

fn main(){
    let o: u64 = 5u64;
    let x = factorial(o);

    // this is rust line comment
    // see it in action
    // e.g. we format x as string with base 2 (binary)
    //   and use it's length to count bits in the number
    let y = format!("{:b}", x).len() as u64;
    // nad now we pri32 it
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
