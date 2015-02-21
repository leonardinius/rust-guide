fn foo(x: &i32) -> i32 {
    println!("{} {} {}", x, (*x) *3, (*x) * 5);
    *x
}

fn main() {
    {
        let mut x = 5i32;

        {
            let y = &x;
            let z = &x;
            println!("1. y *{:p} -> {}", y, y);
            println!("1. z *{:p} -> {}", z, z);
        }

        {
            let i = &mut x;
            *i = *i * 2;
        }

        {
            let y = &x;
            let z = &x;
            println!("2. y *{:p} -> {}", y, y);
            println!("2. z *{:p} -> {}", z, z);
        }

        foo(&x);

    }
}
