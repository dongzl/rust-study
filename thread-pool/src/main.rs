use std::time::SystemTime;
use thread_pool::ThreadPool;

fn main() {
    let pool = ThreadPool::new(2);
    let f1 = || {
        let time = SystemTime::now();
        let result = 1 + 1;
        println!("{:?} result:{result}", time);
    };
    let f2 = || {
        let time = SystemTime::now();
        let result = 99*88*77*76;
        println!("{:?} result:{result}", time);
    };
    let f3 = || {
        let time = SystemTime::now();
        let result = 1000*9899;
        println!("{:?} result:{result}", time);
    };
    pool.execute(f1);
    pool.execute(f2);
    pool.execute(f3);
}