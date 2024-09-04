use std::{sync::{mpsc, Arc, Mutex}, thread};
pub struct Process {
    id: usize,
    clock: usize,
    thread: thread::JoinHandle<()>,
}
pub struct ThreadPool {
    processes: Vec<Process>,
    job_sender: mpsc::Sender<Job>,
    msg_sender: mpsc::Sender<Message>
}

type Job = Box<dyn FnOnce() + Send + 'static>;
type Message = String;

impl Process {
    fn new(id: usize, job_receiver: Arc<Mutex<mpsc::Receiver<Job>>>, msg_receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Process {
        let thread = thread::spawn(move || loop {
            if let Ok(msg) = msg_receiver.lock().unwrap().try_recv() {
                println!("Process {} received a message {}", id, msg);
            }
            if let Ok(job) = job_receiver.lock().unwrap().try_recv() {
                println!("Process {} got a job", id);
                job();
            }
            thread::sleep(std::time::Duration::from_millis(10)); // prevent busy waiting
        });

        let clock = 0;

        Process { id, clock, thread }
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (job_sender, job_receiver) = mpsc::channel();
        let (msg_sender, msg_receiver) = mpsc::channel();
        let job_receiver = Arc::new(Mutex::new(job_receiver));
        let msg_receiver = Arc::new(Mutex::new(msg_receiver));
        let mut processes = Vec::with_capacity(size);
        for id in 0..size {
            processes.push(Process::new(id, Arc::clone(&job_receiver), Arc::clone(&msg_receiver)));
        }

        ThreadPool { processes, job_sender, msg_sender }
    }
    pub fn execute<T>(&self, job: T)
    where
       T: FnOnce() + Send + 'static,
    {
        let job = Box::new(job);
        self.job_sender.send(job).unwrap();
    }
    pub fn send_message(&self, msg: Message) {
        self.msg_sender.send(msg).unwrap();
    }
}