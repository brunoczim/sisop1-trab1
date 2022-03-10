use std::sync::atomic::{AtomicUsize, Ordering::*};
use std::hint;

pub struct Semaphore {
    count: AtomicUsize,
}

impl Semaphore {
    pub fn new(maximum: usize) -> Semaphore {
        Semaphore { count: AtomicUsize::new(maximum) }
    }
    
    pub fn try_enter(&self) -> bool {
        let mut current = self.count.load(Relaxed);
        loop {
            if current == 0 {
                return false;
            } 
            let next = current - 1;
            match self.count.compare_exchange(current, next, Acquire, Relaxed) {
                Ok(_) => return true,
                Err(new_current) => current = new_current,
            }
        }
    }
    
    pub fn enter(&self) {
        while !self.try_enter() {
            hint::spin_loop();
        }
    }
    
    pub fn leave(&self) {
        self.count.fetch_add(1, Release);
    }
}
