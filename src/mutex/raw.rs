use std::sync::atomic::{Ordering::*, AtomicBool};
use std::hint;

pub struct Mutex {
    locked: AtomicBool,
}

impl Mutex {
    pub fn new() -> Mutex {
        Mutex {
            locked: AtomicBool::new(false),
        }
    }

    pub fn try_lock(&self) -> bool {
        self.locked.compare_exchange(false, true, Acquire, Relaxed).is_ok()
    }

    pub fn lock(&self) {
        while !self.try_lock() {
            hint::spin_loop();
        }
    }

    pub fn unlock(&self) {
        self.locked.store(false, Release);
    }
}
