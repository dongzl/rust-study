use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    // // run OK
    // let hello = "Hello Rust";
    //
    // let handle = thread::spawn(move ||{
    //     thread::sleep(Duration::from_millis(1000));
    //
    //     println!("{}, sub thread_id:{:?}", hello, thread::current().id());
    // });
    //
    // println!("{}, main thread_id:{:?}", hello, thread::current().id());
    // handle.join().unwrap();

    // // run error
    // let hello = String::from("Hello Rust");
    //
    // let handle = thread::spawn(move ||{
    //     thread::sleep(Duration::from_millis(1000));
    //
    //     println!("{}, sub thread_id:{:?}", hello, thread::current().id());
    // });
    //
    // println!("{}, main thread_id:{:?}", hello, thread::current().id());
    // handle.join().unwrap();

    // // run OK
    // let a = Arc::new(String::from("Hello Rust"));
    //
    // let b = Arc::clone(&a);
    // let handle = thread::spawn(move ||{
    //     thread::sleep(Duration::from_millis(1000));
    //
    //     println!("{}, sub thread_id:{:?}", b, thread::current().id());
    // });
    //
    // println!("{}, main thread_id:{:?}", a, thread::current().id());
    // handle.join().unwrap();

    // run OK
    let a = String::from("Hello Rust");

    let b = a.clone();
    let handle = thread::spawn(move ||{
        thread::sleep(Duration::from_millis(1000));

        println!("{}, sub thread_id:{:?}", b, thread::current().id());
    });

    println!("{}, main thread_id:{:?}", a, thread::current().id());
    handle.join().unwrap();
}
