//! A minimal implementation of SHA1 for rust.
//!
//! Example:
//!
//! ```rust
//! extern crate sha1;
//! # fn main() {
//!
//! let mut m = sha1::Sha1::new();
//! m.update("Hello World!".as_bytes());
//! assert_eq!(m.hexdigest().as_slice(),
//!            "2ef7bde608ce5404e97d5f042f95f89f1c232871");
//! # }
//! ```


/// `factorial` is  a function that returns a n! from n
///
/// # Arguments
///
/// * `n` - The non-negative number to calculate the factorial to
///
/// # Example 
///
/// ```rust
/// let n: u64 = 5u64;
/// println!("{}! = {}", n, factorial(n)); // prints "5! = 120"
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
    // nad now we print it
    println!("factorial({}) = {} ({:b} / {})", o, x, x, y);

    let q = 15u64;
    if y < q {
        println!("x < {} = {}", q, x);
    }

    let u = if x < 5u64 { 0i } else { 1i };
    println!("u = {}", u);

    let u1 = if x < 5u64 { 0i; } else { 1i; };
    println!("u1 = {}", u1);
}
