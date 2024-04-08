// async fn say_world() {
//     println!("world");
// }

// #[tokio::main]
// async fn main() {

//     let op = say_world();

//     println!("Hello, world!");

//     op.await;
// }

use tokio::task;

#[tokio::main]
async fn main() {
    let v = vec![1, 2, 3];

    task::spawn(async move {
        println!("Here's a vec: {:?}", v);
    });

    // println!("Here's a vec: {:?}", v);
}