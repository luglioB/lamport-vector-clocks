use std::thread;

use lamport_vector_clocks::ThreadPool;
fn main() {
    let pool = ThreadPool::new(1);
    for _ in 0..1 {
        pool.execute(|| {
            run();
        });
    }
    pool.send_message("this is a message".to_string());
    println!("job and message sent");
    thread::sleep(std::time::Duration::from_secs(4));
    println!("done");
}

fn run(){
    println!("I am running");
}