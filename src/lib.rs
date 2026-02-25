use std::thread;
pub struct Threadpool {
    workers: Vec<Worker>,
}

impl Threadpool {
    /// Create a new Threadpool
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panic
    ///
    /// The 'new' function will panic if the size become less than zero
    pub fn new(size: usize) -> Threadpool {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);

        for i in 0..size {
            workers.push(Worker::new(i));
        }
        Threadpool { workers }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

struct Worker {
    id: usize,
    thrd: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        let thrd = thread::spawn(|| {});
        Worker { id, thrd }
    }
}
