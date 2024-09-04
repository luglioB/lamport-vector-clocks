use std::thread;
use ctrlc;
use chrono::Local;
use lamport_vector_clocks::ThreadPool;
fn main() {
    let pool = ThreadPool::new(2);
    for _ in 0..1 {
        pool.execute(|| {
            run();
        });
    }

    ctrlc::set_handler(move || {
        std::process::exit(0);
    }).expect("error setting Ctrl+c handler");

    loop {
        let now = Local::now();
        let timestamp_msg = format!("{}", now);
        pool.send_message(timestamp_msg);
        thread::sleep(std::time::Duration::from_secs(3));
    }
}

fn run(){
    println!("I am running");
}