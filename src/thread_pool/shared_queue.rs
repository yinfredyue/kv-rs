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
            let handle = thread::spawn(move || loop {
                let job = rx.lock().unwrap().recv().unwrap();
                match job {
                    ThreadPoolMessage::Shutdown => break,
                    ThreadPoolMessage::Job(job) => job(),
                }
            });
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
                t.join().unwrap();
            }
        }
    }
}
