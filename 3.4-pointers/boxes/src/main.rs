#[derive(Debug)]
enum List{
    Node(u32, Box<List>),
    Nil
}

#[derive(Debug)]
struct Inches(i32);

fn main() {
    println!("Hello, world!");

    {
        let list = List::Node(0, Box::new(List::Node(1, Box::new(List::Nil))));
        println!("list: {:?}", list);
    }

    {
        let x = Box::new(5i32);
        println!("*{:p} = {}", &x, *x);
    }

    {
        let i = Inches(5);
        let u = i.0 * 2;
        println!("{} inches", u);
    }

    {
        let mut x = Box::new(5i32);
        {
            let y = & mut x;
            **y = **y * 2;
        }

        *x *= 3;

        println!("x = {}", *x);
    }
}
