use std::sync::mpsc;
//use prelude::spawn;
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
pub struct ThreadPool
    {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
    }

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Worker
    {
    _id: usize,
    _jh: Option<thread::JoinHandle<()>>,
    }

impl Worker
    {
    pub fn new(_id: usize, _receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker
        {
        let _thread = thread::spawn(move || loop { 
            let _message = _receiver.lock().unwrap().recv().unwrap();

            match _message
                {
                Message::NewJob(job) => 
                    {
                    println!("Worker {} got af job; executeing.", _id);

                    job();
                    }
                Message::Terminate => 
                    {
                    println!("Worker {} was told to terminate.", _id);
                    
                    break;
                    }
                }
            
        });

        Worker { _id, _jh: Some(_thread), }
        }
    }

impl ThreadPool
    {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(_size: usize) -> ThreadPool
        {
        assert!(_size > 0);

        let(sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        // Pre-allocating space for _size items.
        let mut workers = Vec::with_capacity(_size);

        for _id in 0.._size
            {
            // create some threads and store them in the vector
            workers.push(Worker::new(_id, Arc::clone(&receiver)));
            }

        ThreadPool { workers, sender }
        }
    pub fn execute<F>(&self, _f: F)
        where F: FnOnce() + Send + 'static,
        {
        let job = Box::new(_f);
        self.sender.send(Message::NewJob(job)).unwrap();
        }
    }

impl Drop for ThreadPool
    {
    fn drop(&mut self)
        {
        println!("Sending terminate message to all workers.");
        
        for _ in &self.workers 
            {
            self.sender.send(Message::Terminate).unwrap();
            }

        for worker in &mut self.workers
            {
            println!("Shutting down worker {}", worker._id);

            if let Some(_jh) = worker._jh.take()
                {
                _jh.join().unwrap();
                }
            }
        }
    }

pub enum Message
    {
    NewJob(Job),
    Terminate,
    }