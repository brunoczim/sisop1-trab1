use std::{sync::Arc, cell::Cell, thread};

fn main() {
    let shared = Arc::new(Cell::new(5));
    
    let shared_cloned = shared.clone();
    let join_handle_first = thread::spawn(move || {
        shared_cloned.set(8);
    });
    
    let shared_cloned = shared.clone();
    let join_handle_second = thread::spawn(move || {
        shared_cloned.set(123);
    });
    
    join_handle_first.join().unwrap();
    join_handle_second.join().unwrap();
    
    println!("{}", shared_cloned.get());
}
