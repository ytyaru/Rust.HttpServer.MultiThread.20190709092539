use std::thread;
pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
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
        let mut threads = Vec::with_capacity(size);
        for _ in 0..size {
            // スレッドを生成してベクタに格納する
        }
        ThreadPool {
            threads
        }
    }

    pub fn execute<F>(&self, f: F)
//    pub fn execute<F>(&self, f: F) -> JoinHandle<T>
        where
            F: FnOnce() + Send + 'static
    {

    }
}
