use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{self, JoinHandle},
};

use crate::thread_pool::ThreadPool;

///
pub struct SharedQueueThreadPool {
    handles: Vec<Option<JoinHandle<()>>>,
    tx: mpsc::Sender<ThreadPoolMessage>,
}

enum ThreadPoolMessage {
    Job(Box<dyn FnOnce() + Send + 'static>),
    Shutdown,
}

// https://github.com/pingcap/talent-plan/blob/master/courses/rust/projects/project-4/src/thread_pool/shared_queue.rs
// https://www.reddit.com/r/rust/comments/2ze539/comment/cpi3aiw/?utm_source=share&utm_medium=web2x&context=3
struct ThreadSentinel {
    rx: Arc<Mutex<mpsc::Receiver<ThreadPoolMessage>>>,
}

impl Drop for ThreadSentinel {
    fn drop(&mut self) {
        if thread::panicking() {
            let s = ThreadSentinel {
                rx: self.rx.clone(),
            };
            if let Err(e) = thread::Builder::new().spawn(move || run_tasks(s)) {
                eprintln!("Failed to spawn a thread: {}", e);
            }
        }
    }
}

fn run_tasks(sentinel: ThreadSentinel) {
    loop {
        let job = sentinel.rx.lock().unwrap().recv().unwrap();
        match job {
            ThreadPoolMessage::Shutdown => break,
            ThreadPoolMessage::Job(job) => job(),
        }
    }
}

///
impl ThreadPool for SharedQueueThreadPool {
    fn new(threads: u32) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let mut handles = Vec::new();
        let (tx, rx) = mpsc::channel();

        let rx = Arc::new(Mutex::new(rx));
        for _ in 0..threads {
            let rx = Arc::clone(&rx);
            let sentinel = ThreadSentinel { rx };
            let handle = thread::spawn(move || run_tasks(sentinel));
            handles.push(Some(handle));
        }

        Ok(Self { handles, tx })
    }

    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.tx.send(ThreadPoolMessage::Job(Box::new(job))).unwrap();
    }
}

impl Drop for SharedQueueThreadPool {
    fn drop(&mut self) {
        for _ in 0..self.handles.len() {
            self.tx.send(ThreadPoolMessage::Shutdown).unwrap();
        }

        for t in self.handles.iter_mut() {
            let t = t.take();
            if let Some(t) = t {
                // With respawning, join() returns err. Cannot use unwrap().
                if let Err(err) = t.join() {
                    println!("{:#?}", err);
                }
            }
        }
    }
}
