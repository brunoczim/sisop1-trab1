use std::{collections::HashSet, sync::{Arc, Mutex}, thread};

const TOTAL_THREADS: u64 = 8;
const UPPER_BOUND: u64 = 100_000_000;

fn main() {
    let shared = Arc::new(Mutex::new(HashSet::new()));
    let mut join_handles = Vec::new();

    for id in 1 ..= TOTAL_THREADS {
        let shared = shared.clone();
        let join_handle = thread::spawn(move || {
            let mut number = id;
            while number <= UPPER_BOUND {
                if is_prime(number) {
                    // Locks here and acquires the guard
                    let mut guard = shared.lock().unwrap();
                    // Uses the resource (a HashSet) while locked
                    (*guard).insert(number);
                    // Automatically unlocks here where the guard is dropped
                }
                number += TOTAL_THREADS;
            }
        });
        join_handles.push(join_handle);
    }

    for join_handle in join_handles {
        join_handle.join().unwrap();
    }

    println!("Found {} primes", shared.lock().unwrap().len());
}

fn is_prime(number: u64) -> bool {
    if number < 2 {
        return false;
    }
    if number == 2 {
        return true;
    }
    if number % 2 == 0 {
        return false;
    }
    let mut attempt = 3;
    while attempt * attempt <= number {
        if number % attempt == 0 {
            return false;
        }
        attempt += 2;
    }
    true
}
