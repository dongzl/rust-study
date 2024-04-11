use std::sync::{Arc, Mutex};
use std::thread;

// fn main() {
//     let m = Mutex::new(5);
//     {
//         let mut num = m.lock().unwrap();
//         *num = 6;
//     }
//
//     println!("m = {:?}", m);
// }

// fn main() {
//     let counter = Arc::new(Mutex::new(10));
//     let mut handlers = vec![];
//
//     for _ in 0..10 {
//         let counter = Arc::clone(&counter);
//         let handle = thread::spawn(move || {
//            let mut num = counter.lock().unwrap();
//             *num += 1;
//         });
//         handlers.push(handle);
//     }
//
//     for handle in handlers {
//         handle.join().unwrap();
//     }
//
//     println!("Result: {}", *counter.lock().unwrap());
// }

fn main() {
    // let counter = Arc::new(Mutex::new(10));
    // let mut handlers = vec![];
    //
    // for _ in 0..10 {
    //     let counter = Arc::clone(&counter);
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();
    //         *num += 1;
    //     });
    //     handlers.push(handle);
    // }
    //
    // for handle in handlers {
    //     handle.join().unwrap();
    // }
    //
    // println!("Result: {}", *counter.lock().unwrap());

    // let counter = Arc::new(10);
    // let mut handlers = vec![];
    //
    // for _ in 0..10 {
    //     let counter = Arc::clone(&counter);
    //     let handle = thread::spawn(move || {
    //         let mut num = *counter;
    //         num += 1;
    //     });
    //     handlers.push(handle);
    // }
    //
    // for handle in handlers {
    //     handle.join().unwrap();
    // }
    //
    // println!("Result: {}", *counter);

    let s = Arc::new(3);
    for _ in 0..10 {
        let s = Arc::clone(&s);
        let handle = thread::spawn(move || {
            s = s + 1;
            println!("{}", s)
        });
    }
}
