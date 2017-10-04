use std::thread;

struct Worker {
    id: usize,
    handle: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        Worker {
            id,
            handle: thread::spawn(|| {}),
        }
    }
}

// struct Job;

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    /// `size` is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);
        for i in 0..size {
            workers.push(Worker::new(i));
        }
        ThreadPool { workers }
    }
    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static {}
}
