use std::thread;
pub struct ThreadPool {
    workers: Vec<Worker>,
}
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
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id));
        }
        ThreadPool {
            workers
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
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});
        Worker {
            id,
            thread,
        }
    }
}
