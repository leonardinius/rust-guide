extern crate phrases;

fn main(){
    main1();
    main2();
}

fn main1() {
    use phrases::english::greetings;
    use phrases::english::farewells;
    println!("Hello in English: {}", greetings::hello());
    println!("Goodbye in English: {}", farewells::goodbye());
}

fn main2() {
    use phrases::english::{greetings,farewells};
    println!("Hello in English: {}", greetings::hello());
    println!("Goodbye in English: {}", farewells::goodbye());
}
