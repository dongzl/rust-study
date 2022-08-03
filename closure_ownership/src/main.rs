fn main() {
    let mut a = 1;
    let mut inc = || {a+=1;a};
    inc();
    inc();
    println!("now a is {}", a); // output 3

    let s = String::from("test");
    let f = || {let _s = s;println!("{}", _s)};
    f();
    //f(); // can't run, because s dropped

    let mut s = String::from("董泽润的技术笔记");
    let f = || {s.push_str("不错");};
    test(f);

    let s = String::from("test");
    let f = move || {println!("{}", s)}; // s move to f
    f();
    f();
}

fn test<T>(mut f: T) where
    T: FnMut()
{
    f();
}
