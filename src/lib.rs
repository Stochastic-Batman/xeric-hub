use std::{
    sync::{Arc, mpsc, Mutex}, 
    thread,
};


type Job = Box<dyn FnOnce() + Send + 'static>;


#[derive(Debug)]
pub struct PoolCreationError;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}


impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || { 
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {id} got a job; executing...");
                job();
            }
        });
        
        Worker { id, thread }
    }    
}

impl ThreadPool {
    /// Create a new ThreadPool.
    /// 
    /// The `size` is the number of threads in the pool.
    ///
    /// Returns a `Result` containing the `ThreadPool` if successful, 
    /// or a `PoolCreationError` if the `size` is 0.
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size == 0 {
            return Err(PoolCreationError);
        }

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Ok(ThreadPool { workers, sender })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}
