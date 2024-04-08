// struct Solution;

// impl Solution {
//     pub fn move_zeroes(num: &mut Vec<i32>) {
//         let mut i = 0;
//         let mut j = 0;
//         while j < num.len() {
//             if num[j] != 0 {
//                 num[i] = num[j];
//                 i += 1;
//             }
//             j += 1;
//         }
//         let mut k = i;
//         while k < num.len() {
//             num[k] = 0;
//             k += 1;
//         }
//     }
// }

// fn main() {
//     let mut vec: Vec<i32> = vec![0,1,0,3,12];
//     Solution::move_zeroes(&mut vec);
//     println!("{:?}", vec);
// }

// fn main(){
//     let mut a = 1;
//     let mut inc = || {a+=1;a};
//     inc();
//     inc();
//     println!("now a is {}", a);
// }

// fn main(){
//     let s = String::from("test");
//     let f = || {let _s = s;println!("{}", _s)};
//     f();
//     f();
// }

// fn main(){
//     let mut s = String::from("test");
//     let mut f = || {s.push('a');println!("{}", s)};
//     f();
//     f();
// }

// fn main(){
//     let s = String::from("test");
//     let f = move || {println!("{}", s)};
//     f();
//     f();
// }

// fn test<T>(f: T) where
//     T: Fn()
// {
//     f();
// }

// fn main() {
//     let s = String::from("董泽润的技术笔记");
//     let f = || {println!("{}", s);};
//     test(f);
// }

// fn test<T>(mut f: T) where
//     T: FnMut()
// {
//     f();
// }

// fn main() {
//     let mut s = String::from("董泽润的技术笔记");
//     let f = || {s.push_str("不错");};
//     test(f);
// }

// fn test<T>(f: T) where
//     T: Fn()
// {
//     f();
//     f();
// }

// fn main() {
//     let s = String::from("董泽润的技术笔记");
//     // let f = || {let _ = s;};
//     let f = move || {println!("s is {}", s);};
//     test(f);
// }

// fn call(f: fn()) {    // function pointer
//     f();
// }

// fn main() {
//     let a = 1;

//     let f = || println!("abc");     // anonymous function
//     let c = || println!("{}", &a);  // closure

//     call(f);
//     call(c);
// }

// fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
//     Box::new(|x| x + 1)
// }

// fn main() {
//     let f = returns_closure();
//     println!("res is {}", f(11));
// }

// fn returns_closure() -> impl Fn(i32) -> i32 {
//     |x| x + 1
// }

// fn main() {
//     let f = returns_closure();
//     println!("res is {}", f(11));
// }

fn test() -> impl FnMut(char) {
    let mut s = String::from("董泽润的技术笔记");
    move |c| { s.push(c); }
}

fn main() {
    let mut c = test();
    c('d');
    c('e');
}