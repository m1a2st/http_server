use std::{sync::mpsc, thread};
use std::sync::mpsc::channel;
use super::job::Job;
use super::worker::Worker;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::Sender::channel();

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id));
        }
        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
    {}
}
