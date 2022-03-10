use std::sync::atomic::{AtomicUsize, Ordering::*};
use std::hint;

const LOCKED_WRITE: usize = 0;
const UNLOCKED: usize = 1;

pub struct RwLock {
    count: AtomicUsize,
}

impl RwLock {
    pub fn new() -> RwLock {
        RwLock { count: AtomicUsize::new(UNLOCKED) }
    }
    
    pub fn try_lock_read(&self) -> bool {
        let mut current = self.count.load(Relaxed);
        loop {
            if current == LOCKED_WRITE {
                return false;
            } 
            let next = current + 1;
            match self.count.compare_exchange(current, next, Acquire, Relaxed) {
                Ok(_) => return true,
                Err(new_current) => current = new_current,
            }
        }
    }
    
    pub fn lock_read(&self) {
        while !self.try_lock_read() {
            hint::spin_loop();
        }
    }
    
    pub fn unlock_read(&self) {
        self.count.fetch_sub(1, Release);
    }
    
    pub fn try_lock_write(&self) -> bool {
        self.count.compare_exchange(UNLOCKED, LOCKED_WRITE, Acquire, Relaxed).is_ok()
    }
    
    pub fn lock_write(&self) {
        while !self.try_lock_write() {
            hint::spin_loop();
        }
    }
    
    pub fn unlock_write(&self) {
        self.count.store(UNLOCKED, Release);
    }
}
