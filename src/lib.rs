use std::thread;
pub struct Process {
    id: usize,
    thread: thread::JoinHandle<()>,
}
pub struct ThreadPool {
    processes: Vec<Process>,
}

impl Process {
    fn new(id: usize) -> Process {
        let thread = thread::spawn(|| {});

        Process { id, thread }
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut processes = Vec::with_capacity(size);
        for id in 0..size {
            processes.push(Process::new(id));
        }

        ThreadPool { processes }
    }
    pub fn execute<T>(&self, t: T)
    where
       T: FnOnce() + Send + 'static,
    {}
}