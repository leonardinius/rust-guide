#[deriving(Show)]
enum List{
    Node(u32, Box<List>),
    Nil
}

#[deriving(Show)]
struct Inches(int);

fn main() {
    println!("Hello, world!");

    {
        let list = List::Node(0, box List::Node(1, box List::Nil));
        println!("list: {}", list);
    }

    {
        let x = box 5i;
        println!("*{:p} = {}", &x, *x);
    }

    {
        let i = Inches(10);
        let u = i.0 * 2;
        println!("{} inches", u);
    }

    {
        let mut x = box 5i;
        {
            let y = & mut x;
            **y = **y * 2;
        }

        *x *= 3;

        println!("x = {}", *x);
    }
}
