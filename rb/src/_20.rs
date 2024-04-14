const ROOT_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>Hello!</title>
</head>
<body>
<h1>Hello!</h1>
<p>Hi from Rust</p>
</body>
</html>"#;

const NOT_FOUND_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>Hello!</title>
</head>
<body>
<h1>Oops!</h1>
<p>Sorry, I don't know what you're asking for.</p>
</body>
</html>"#;

pub mod server {
    use std::{
        io::{prelude::*, BufReader},
        net::{TcpListener, TcpStream},
        thread,
        time::Duration,
    };

    use super::pool::ThreadPool;
    use super::{NOT_FOUND_HTML, ROOT_HTML};

    pub fn listen() {
        const PORT: &str = "7879";
        let listener = TcpListener::bind(format!("127.0.0.1:{PORT}")).unwrap();
        println!("Server istening on port {PORT}");

        let pool = match ThreadPool::build(4) {
            Ok(pool) => pool,
            Err(_) => {
                println!("Failed to create a thread pool.");
                return;
            }
        };

        for stream in listener.incoming().take(2) {
            let stream = stream.unwrap();

            pool.execute(|| {
                handle_connection(stream);
            });
        }

        println!("Shutting down.");
    }

    fn handle_connection(mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let request_line = buf_reader.lines().next().unwrap().unwrap();

        let (status_line, response_body) = match &request_line[..] {
            "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", ROOT_HTML),
            "GET /sleep HTTP/1.1" => {
                thread::sleep(Duration::from_secs(5));
                ("HTTP/1.1 200 OK", ROOT_HTML)
            }
            _ => ("HTTP/1.1 404 NOT FOUND", NOT_FOUND_HTML),
        };

        let response = format!(
            "{status_line}\r\nContent-Length: {body_len}\r\n\r\n{response_body}",
            body_len = response_body.len(),
        );
        stream.write_all(response.as_bytes()).unwrap();
    }
}

pub mod pool {
    use std::{
        sync::{mpsc, Arc, Mutex},
        thread,
    };

    type Job = Box<dyn FnOnce() + Send + 'static>;

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

    pub enum PoolCreationError {
        ZeroSizedPool,
    }

    pub struct ThreadPool {
        workers: Vec<Worker>,
        sender: Option<mpsc::Sender<Job>>,
    }

    impl ThreadPool {
        /// Create a new ThreadPool.
        ///
        /// The size is the number of threads in the pool.
        ///
        /// # Panics
        ///
        /// The `new` function will panic if the size is zero.
        pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
            if size == 0 {
                Err(PoolCreationError::ZeroSizedPool)
            } else {
                let (sender, receiver) = mpsc::channel();

                let mut workers = Vec::with_capacity(size);

                let receiver = Arc::new(Mutex::new(receiver));

                for id in 0..size {
                    workers.push(Worker::new(id, Arc::clone(&receiver)));
                }

                Ok(ThreadPool {
                    workers,
                    sender: Some(sender),
                })
            }
        }

        pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
        {
            let job = Box::new(f);

            self.sender.as_ref().unwrap().send(job).unwrap();
        }

        pub fn drop(&mut self) {
            drop(self.sender.take());
            for worker in &mut self.workers {
                println!("Shutting down worker {}", worker.id);

                if let Some(thread) = worker.thread.take() {
                    thread.join().unwrap();
                }
            }
        }
    }
}
