use std::{sync::{mpsc, Arc, Mutex}, thread};
pub struct Process {
    id: usize,
    thread: thread::JoinHandle<()>,
}
pub struct ThreadPool {
    processes: Vec<Process>,
    sender: mpsc::Sender<Job>
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl Process {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Process {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Process {} got a job", id);
            job();
        });

        Process { id, thread }
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut processes = Vec::with_capacity(size);
        for id in 0..size {
            processes.push(Process::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { processes, sender }
    }
    pub fn execute<T>(&self, job: T)
    where
       T: FnOnce() + Send + 'static,
    {
        let job = Box::new(job);
        self.sender.send(job).unwrap();
    }

}