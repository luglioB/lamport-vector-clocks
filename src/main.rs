//use std::sync::mpsc;
//use std::thread;
use lamport_vector_clocks::ThreadPool;
fn main() {
    //let (tx,rx) = mpsc::channel();
    //let v = vec![1,2,3];
    /* 
        thread::spawn(move || {
        let mut v_copy = v.clone();
        for i in 0..v_copy.len() {
            v_copy[i] *= 2;
        }
        
        tx.send(v_copy).unwrap();
        let received = rx.recv().unwrap();
        println!("Got: {:?}", received);
    });
    */
    let pool = ThreadPool::new(1);
    for i in 0..1 {
        pool.execute(|| {
            run();
        })
    }
//    handle.join().unwrap();
}

fn run(){
    println!("I am running");
}