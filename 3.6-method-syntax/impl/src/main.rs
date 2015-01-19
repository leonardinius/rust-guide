fn main() {
    println!("Hello, world!");

    #[derive(Show)]
    struct Circle {
        x : f64,
        y : f64,
        radius: f64,
    }

    impl Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * ( self.radius * self.radius )
        }

        fn new(x: f64, y: f64, r: f64) -> Circle {
            Circle {x: x, y: y, radius: r}
        }
    }

    {
        let c = Circle { x: 0.0, y: 0.0, radius: 3.4 };
        println!("{}", c.area());
    }

    {
        let c = Circle::new(0.0, 0.0, 3.4);
        println!("{:?}, {}", c, c.area());
    }
}
