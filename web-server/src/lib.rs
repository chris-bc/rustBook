//! # Lightweight Web Server
//! 
//! The application currently shuts down after serving two requests in order to
//! test the thread coordination and shutdown process. To remove this limit modify
//! ```fn main()``` in ```main.rs```. Find the following line:
//! ```
//! for stream in listener.incoming().take(2) { // Limit to 2 connections to test thread shutdown
//!     // Snip
//! }
//! ```
//! Replace this with
//! ```
//! for stream in listener.incoming() {
//!     // Snip
//! }
//! ```
//! 
//! ## ToDo
//! 
//! * Add more documentation to ```ThreadPool``` and its public methods;
//! * Add tests of the library's functionality;
//! * Change calls to ```unwrap()``` to do more robust error handling;
//! * Use ```ThreadPool``` to perform some task other than serving web requests;
//! * Find a thread pool crate on [crates.io](https://crates.io) and implement a
//!   similar web server using the crate instead. Then compare its API and robustness
//!   to the thread pool we implemented.
//! 

use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl Drop for ThreadPool {
    fn drop(& mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    /// 
    /// The size is the number of threads in the pool.
    /// 
    /// # Panics
    /// 
    /// The `new` function will panic if the size is zero.
    /// 
    /// # TODO
    /// 
    /// Create a ```build``` function for use instead of ```new```, using the following signature
    /// ```
    /// pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError>
    /// ```
    /// 
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // Create some threads and store them in the vector
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender: Some(sender) }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            
            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread)
        }
    }
}