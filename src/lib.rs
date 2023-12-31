
use std::{
    sync::{mpsc, Arc, Mutex},
    thread, num::ParseIntError,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
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
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
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
            thread: Some(thread),
        }
    }
}



/// A unsigned 16-bit integer (0-65535) identifying the port address.
struct PortNumber{
    port: u16
}


impl PortNumber{
    /// Checks if the port is within the reserved range for known TCP/IP services.
    /// 
    /// Note: This will prevent the user from selecting a port within this range range but
    /// does not check if the port is actually unused.

    fn is_reserved(&self) -> bool{
        if self.port <= 1023 {
            println!("This port address is within the reserved range (0-1023), please select another port number");
            true
        }else {
            false
        }
    }
}

/// Asks the user to specify a port number for the HTTP server.
/// 
/// This function will create a PortNumber type using the entered value that will force the constraints and validation needed.
/// 
/// If an invalid port is entered, the user will be guided to reattempt.
pub fn getvalidport() -> String{

    println!("Enter the port number");

    let mut valid_port = false;

    let mut set_port = PortNumber{port: 7878}; // default port

    while !valid_port{

        let mut port_address = String::new();
        let _input = std::io::stdin().read_line(&mut port_address).unwrap();

        let result: Result<u16, ParseIntError> = port_address.trim().parse();

        match result{
            Ok(port) => {
                set_port.port = port;
                if set_port.is_reserved(){
                    ()
                }else{
                    set_port.port = port;
                    valid_port = true;
                }
            },
            Err(error) => { 
                println!("Entered value [{}] is not a valid port number, please enter a number between 1023 and 65535.\n[Error Message]: {}",port_address, error);
                ()
            
            },
        }


    }

    set_port.port.to_string()

}