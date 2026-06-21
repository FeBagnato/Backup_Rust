use std::sync::{Mutex, Condvar};

pub struct Semaphore {
    count: Mutex<usize>,
    cvar: Condvar,
    max: usize,
}

impl Semaphore {
    pub fn new(max: usize) -> Self {
        Self { count: Mutex::new(0), cvar: Condvar::new(), max }
    }
    pub fn acquire(&self) -> SemaphoreGuard<'_>{
        let mut count = self.count.lock().unwrap();
        while *count >= self.max {
            count = self.cvar.wait(count).unwrap();
        }
        *count += 1;

        SemaphoreGuard { semaphore: self }
    }
    fn release(&self) {
        let mut count = self.count.lock().unwrap();
        *count -= 1;
        self.cvar.notify_one();
    }
}

pub struct SemaphoreGuard<'a> {
    semaphore: &'a Semaphore
}

impl Drop for SemaphoreGuard<'_> {
    fn drop(&mut self) {
        self.semaphore.release();
    }
}

