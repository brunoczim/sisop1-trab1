use std::sync::atomic::{AtomicUsize, Ordering::*};
use std::hint;

const LEAVING_FLAG: usize = !(usize::MAX >> 1);

pub struct Barrier {
    total: usize,
    count: AtomicUsize,
}

impl Barrier {
    pub fn new(total: usize) -> Barrier {
        Barrier {
            total,
            count: AtomicUsize::new(0),
        }
    }
    
    pub fn wait(&self) {
        let entering_count = self.enter();
        self.leave(entering_count);
    }
    
    fn enter(&self) -> usize {
        let mut count = self.count.load(Relaxed);
        
        loop {
            if count & LEAVING_FLAG == 0 && count < self.total {
                match self.count.compare_exchange(count, count + 1, Acquire, Relaxed) {
                    Ok(_) => return count,
                    Err(new_count) => count = new_count,
                }
            } else {
                hint::spin_loop();
                count = self.count.load(Relaxed);
            }
        }
    }
    
    fn leave(&self, entering_count: usize) {
        if entering_count + 1 == self.total {
            self.count.fetch_or(LEAVING_FLAG, Relaxed);
        } else {
            while self.count.load(Relaxed) & LEAVING_FLAG == 0 {
                hint::spin_loop();
            }
        }
        
        let leaving_count = self.count.fetch_sub(1, Release);
        if leaving_count - 1 == LEAVING_FLAG {
            self.count.store(0, Relaxed);
        }
    }
}
