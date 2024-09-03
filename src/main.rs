use std::thread;

use lamport_vector_clocks::ThreadPool;
fn main() {
    let pool = ThreadPool::new(1);
    for _ in 0..1 {
        pool.execute(|| {
            run();
        });
    }

    thread::sleep(std::time::Duration::from_secs(1));
}

fn run(){
    println!("I am running");
}