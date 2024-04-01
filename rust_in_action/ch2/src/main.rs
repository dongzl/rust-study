// fn main() {
//     println!("Hello, world!");
// }

// fn main() {
//     let a = 10;
//     let b: i32 = 20;
//     let c = 30i32;
//     let d = 30_i32;
//     let e = add(add(a, b), add(c, d));
//     println!("( a + b ) + ( c + d ) = {}", e);
// }
//
// fn add(i: i32, j: i32) -> i32 {
//     i + j
// }


// fn main() {
//     let twenty = 20;
//     let twenty_one: i32 = 21;
//     let twenty_two = 22i32;
//
//     let addition = twenty + twenty_one + twenty_two;
//     println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);
//
//     let one_million: i64 = 1_000_000;
//     println!("{}", one_million.pow(2));
//
//     let forty_twos = [42.0, 42f32, 42.0_f32];
//     println!("{:02}", forty_twos[0]);
// }

// fn main() {
//     let three = 0b11;
//     let thirty = 0o36;
//     let three_hundred = 0x12C;
//     println!("base 10: {} {} {}", three, thirty, three_hundred);
//     println!("base 2: {:b} {:b} {:b}", three, thirty, three_hundred);
//     println!("base 8: {:o} {:o} {:o}", three, thirty, three_hundred);
//     println!("base 16: {:x} {:x} {:x}", three, thirty, three_hundred);
// }

// fn main() {
//     let a: i32 = 10;
//     let b: u16 = 100;
//     if a < b {
//         println!("Ten is less than one hundred.");
//     }
// }

// fn main() {
//     let a: i32 = 10;
//     let b: u16 = 100;
//     if a < (b as i32) {
//         println!("Ten is less than one hundred.");
//     }
// }

// use std::convert::TryInto;
//
// fn main() {
//     let a: i32 = 10;
//     let b: u16 = 100;
//     let b_ = b.try_into().unwrap();
//     if a < b_ {
//         println!("Ten is less than one hundred.");
//     }
// }

// use std::time::{Duration, Instant};                //<1>
//
// fn main() {
//     let mut count = 0;
//     let time_limit = Duration::new(1,0);            //<2>
//     let start = Instant::now();                     //<3>
//
//     while (Instant::now() - start) < time_limit {   //<4>
//         count += 1;
//     }
//     println!("{}", count);
// }

// fn main() {
//     //let needle = 42;
//     let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];
//
//     for item in &haystack {
//         let result = match item {
//             42 | 132 => "hit!",
//             _ => "miss",
//         };
//
//         if result == "hit!" {
//             println!("{}: {}", item, result);
//         }
//     }
// }

// fn main() {
//     let a = 42;
//     let r = &42;
//     let b = a + *r;
//     println!("a + a = {}", b);
// }

// fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
//     *i + *j
// }

// fn main() {
//     let a = 10;
//     let b = 20;
//     let res = add_with_lifetimes(&a, &b);
//     println!("{}", res);
// }

use std::ops::Add;
use std::time::Duration;

fn add<T: Add<Output = T>>(i: T, j: T) -> T {
    i + j
}

fn main() {
    let floats = add(1.2, 3.4);
    let ints = add(10, 20);
    let durations = add(Duration::new(5, 0), Duration::new(10, 0));

    println!("{}", floats);
    println!("{}", ints);
    println!("{:?}", durations);
}