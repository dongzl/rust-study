// fn main() {
//     // let mut s = String::from("hello");
//     // s.push_str(", world!"); // push_str() 在字符串后追加字面值
//     // println!("{}", s); // 将打印 `hello, world!`

//     // let s1 = String::from("hello");
//     // let s2 = s1;
//     // println!("{}, world!", s1);

//     // let x: &str = "hello, world";
//     // let y = x;
//     // println!("{},{}",x,y);

//     // let s1 = String::from("hello");
//     // let s2 = s1.clone();
//     // println!("s1 = {}, s2 = {}", s1, s2);

//     let s = String::from("hello");  // s 进入作用域
//     takes_ownership(s);             // s 的值移动到函数里 ...
//                                     // ... 所以到这里不再有效
//     let x = 5;                      // x 进入作用域
//     makes_copy(x);                  // x 应该移动函数里，
//                                     // 但 i32 是 Copy 的，所以在后面可继续使用 x
    
//     //println!("在move进函数后继续使用s: {}",s);
// } // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
//   // 所以不会有特殊操作

// fn takes_ownership(some_string: String) { // some_string 进入作用域
//     println!("{}", some_string);
// } // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

// fn makes_copy(some_integer: i32) { // some_integer 进入作用域
//     println!("{}", some_integer);
// } // 这里，some_integer 移出作用域。不会有特殊操作

// fn main() {
//     println!("start");
//     // code 1:
//     let a = 1;
//     let _b = a;
//     let _c = a;

//     // code 2:
//     let d = String::from("hello");
//     let _e = d;
//     let _f = d;
// }

// #[derive(Debug)]
// struct MyString(String);

// impl MyString {
//     fn from(name: &str) -> Self {
//         MyString(String::from(name))
//     }    
// }

// struct MyData {
//     data: MyString,
// }

// impl Drop for MyString {
//     fn drop(&mut self) {
//         println!("Dropping MyString with value: {:?}", self.0);
//     }
// }

// impl Drop for MyData {
//     fn drop(&mut self) {
//         println!("Dropping MyData with value: {:?}", self.data);
//     }
// }

// fn main() {
//     {
//         let _ = MyData {
//             data: MyString::from("not used"),
//         };
//         let _wrapper = MyData {
//             data: MyString::from("used as variable"),
//         };
//         println!("End of the scope inside main.");
//     }

//     println!("End of the scope.");
// }

// fn main() {
//     println!("start");
//     let a = String::from("hello");
//     let d = &a;
//     // 等效于
//     // let ref d = a;
//     let _e = d;
//     let _f = d;
// }

// #[derive(Debug)]
// struct MyString(String);

// impl MyString {
//     fn from(name: &str) -> Self {
//         MyString(String::from(name))
//     }    
// }

// struct MyData {
//     data: MyString,
// }

// struct MyDataRef<'a> {
//     reference: &'a MyData,
// }

// impl Drop for MyString {
//     fn drop(&mut self) {
//         println!("Dropping MyString with value: {:?}", self.0);
//     }
// }

// impl Drop for MyData {
//     fn drop(&mut self) {
//         println!("Dropping MyData with value: {:?}", self.data);
//     }
// }

// impl Drop for MyDataRef<'_> {
//     fn drop(&mut self) {
//         println!("Dropping MyDataRef");
//     }
// }

// fn main() {
//     {
//         let a = MyData {
//             data: MyString::from("used as variable"),
//         };
//         let b = MyDataRef { reference: &a };
//         let c = MyDataRef { reference: &a };
//         println!("End of the scope inside main.");
//     }

//     println!("End of the scope.");
// }

// use std::cell::RefCell;

// fn main() {
//     let value = RefCell::new(5);
//     let borrowed = value.borrow();
//     println!("Before mutation: {}", *borrowed);
//     drop(borrowed);
//     {
//         let mut borrowed_mut = value.borrow_mut();
//         *borrowed_mut += 1;
//     }

//     let borrowed = value.borrow();
//     println!("After mutation: {}", *borrowed);
// }

// fn longest<'a>(str1:  &'a str, str2: &'a str) -> &'a str {
//     if str1.len() > str2.len() {
//         str1
//     } else {
//         str2
//     }
// }

// fn main() {
//     let str1 = "hello";
//     let str2 = "world！";

//     let result = longest(str1, str2);
//     println!("The longest string is: {}", result);
// }

// fn get_longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
//     if str1.len() > str2.len() {
//         str1
//     } else {
//         str2
//     }
// }

// fn main() {
//     let result;
//     {
//         let str1 = String::from("hello");
//         let str2 = "world!";
//         result = get_longest(str1.as_str(), str2);
//     }
//     println!("The longest string is: {}", result);
// }

// use std::{rc::Rc, cell::RefCell};

// struct Owner {
//     name: String,
//     gadgets: RefCell<Vec<Rc<Gadget>>>
// }

// struct Gadget {
//     id: i32,
//     owner: Rc<Owner>
// }

// fn main() {
//     let gadget_owner : Rc<Owner> = Rc::new(
//             Owner { name: String::from("Gadget Man"), gadgets: RefCell::new(vec![]) }
//     );
//     // 两个工具，都有同一个所有者
//     let gadget1 = Rc::new(Gadget { id: 1, owner: gadget_owner.clone() });
//     let gadget2 =Rc::new(Gadget { id: 2, owner: gadget_owner.clone() });

//     gadget_owner.gadgets.borrow_mut().push(gadget1.clone());
//     gadget_owner.gadgets.borrow_mut().push(gadget2.clone());
//     // 释放gadget_owner的引用计数，保留工具的owner引用计数
//     drop(gadget_owner);

//     println!("strong count of gadget1: {}", Rc::strong_count(&gadget1));
//     // strong count of gadget1: 2

//     println!("strong count of gadget1.owner: {}", Rc::strong_count(&gadget1.owner));
//     // strong count of gadget1.owner: 2

//     // 释放gadget1的引用计数，正常没有引用循环的话，owner对应的引用计数也需要释放
//     // 但是gadget1的owner的引用计数不会减一，导致内存泄漏
//     drop(gadget1);

//     println!("strong count of gadget2.owner: {}", Rc::strong_count(&gadget2.owner));
//     // strong count of gadget2.owner: 2
// }

use std::rc::Rc;

// fn main() {
//     let a = Rc::new(String::from("hello"));
//     let _b = Rc::clone(&a);
//     {
//         let _b = Rc::clone(&a);
//         println!("reference count {}", Rc::strong_count(&a));
//     }

//     println!("reference count {}", Rc::strong_count(&a)); // 2
// }

// fn main() {
//     let mut a = Rc::new(String::from("hello"));
//     let b = Rc::clone(&a);

//     (*Rc::make_mut(&mut a)).push_str(" world");
//     println!("{} {}",  a, b);

//     let c = Rc::clone(&a);
//     println!("{} {} {}",  a, b, c);
// }

use std::sync::Arc;
use std::thread;

fn main() {
    let val = Arc::new(5);

    for _ in 0..3 {
        let val = Arc::clone(&val);
        thread::spawn(move || {
            let v = *val.as_ref() + 1;
            println!("{v:?}");
        });
    }
    thread::sleep(std::time::Duration::from_secs(1));
}