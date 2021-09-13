use clap::{AppSettings, Clap};
use std::net::TcpStream;
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;
use exitcode;

#[derive(Clap)]
#[clap(version = "1.0", author = "Josh M. <https://github.com/joshmuente>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(short, long, takes_value = true, default_value = "127.0.0.1")]
    host: String,
    #[clap(short, long, takes_value = true, default_value = "1")]
    from_port: usize,
    #[clap(short, long, takes_value = true, default_value = "65535")]
    to_port: usize,
    #[clap(short, long, takes_value = true, default_value = "10")]
    amount_thread: usize
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for _id in 0..size {
            workers.push(Worker::new(Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    job();
                }
                Message::Terminate => {
                    break;
                }
            }
        });

        Worker {
            thread: Some(thread),
        }
    }
}

fn main() {
    let opts: Opts = Opts::parse();
    if opts.from_port > 65535 || opts.to_port > 65535 || opts.from_port >= opts.to_port {
        println!("port must be between 0 and 65535");
        std::process::exit(exitcode::USAGE);
    }

    if opts.amount_thread <= 0 {
        println!("thread amount must be greater 1");
        std::process::exit(exitcode::USAGE);
    }

    let pool = ThreadPool::new(opts.amount_thread);

    for i in opts.from_port..opts.to_port+1 {
        let x = i.clone();
        let host = opts.host.clone();
        pool.execute(move || {
            check_port(host, x as i32)
        });
    }

    drop(pool);

    std::process::exit(exitcode::OK);
}

fn check_port(host: String, port: i32) {
    let stream = TcpStream::connect(format!("{}:{}", host, port));
    if stream.is_ok() {
        println!("{} open", port);
    }
}


