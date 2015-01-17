struct Car {
    name : String,
}

struct Whhel {
    size : i32,
    owner : Car,
}

fn main () {
    let car = Car {name: "DeLorean".to_string() };

    for _ in range(0, 4) {
        Wheel { size : 360, owner : car };
    }
}
