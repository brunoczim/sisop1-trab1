use std::sync::atomic::{Ordering::*, AtomicI32};

pub fn fetch_mul(atomic_int: &AtomicI32, factor: i32) -> i32 {
    let mut current = atomic_int.load(Acquire);
    
    loop {
        match atomic_int.compare_exchange(
            current,
            current * factor,
            AcqRel,
            Acquire,
        ) {
            Ok(_) => return current,
            Err(new_current) => current = new_current,
        }
    }
}
