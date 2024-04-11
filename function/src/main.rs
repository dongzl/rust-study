// fn main() {
//     another_function(5, 6.1);
//
//     let x = plus_five(5);
//
//     println!("The value of x is: {}", x);
//
//     let x = plus_or_minus(6);
//
//     println!("The value of x is: {}", x);
// }
//
// fn plus_or_minus(x:i32) -> i32 {
//     if x > 5 {
//         return x - 5
//     }
//     x + 5
// }
//
// fn plus_five(x:i32) -> i32 {
//     x + 5
// }
//
// fn another_function(x: i32, y: f32) {
//     println!("The value of x is: {}", x);
//     println!("The value of y is: {}", y);
// }

trait Cook {
    fn start(&self);
}

trait Wash {
    fn start(&self);
}

struct Chef;

impl Cook for Chef {
    fn start(&self) {
        println!("Cook::start");
    }
}

impl Wash for Chef {
    fn start(&self) {
        println!("Wash::start");
    }
}

fn main() {
    let me = Chef;
    Wash::start(&me);
    Cook::start(&me);
    <Chef as Cook>::start(&me);
}