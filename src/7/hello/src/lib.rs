use std::thread;
use std::sync::mpsc;
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}
struct Job;
impl ThreadPool {
    /// スレッドプールを生成する。
    ///
    /// * size: プール数
    ///
    /// # panic
    ///
    /// sizeが0ならパニックする。
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, receiver));
        }
        ThreadPool {
            workers,
            sender,
        }
    }
    pub fn execute<F>(&self, f: F)
//    pub fn execute<F>(&self, f: F) -> JoinHandle<T>
        where
            F: FnOnce() + Send + 'static
    {

    }
}
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}
impl Worker {
    fn new(id: usize, receiver: mpsc::Receiver<Job>) -> Worker {
        let thread = thread::spawn(|| {});
        Worker {
            id,
            thread,
        }
    }
}
