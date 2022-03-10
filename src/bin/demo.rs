use std::thread;
use std::time::Duration;

fn main() {
    let mut join_handles = Vec::new();

    for num in 0 .. 16 {
        let join_handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            println!("Exiting thread {}...", num);
            let triple = num * 3;
            triple
        });
        join_handles.push(join_handle);
    }

    let mut sum_of_triples = 0;
    for join_handle in join_handles {
        sum_of_triples += join_handle.join().unwrap();
    }

    println!("Sum: {}", sum_of_triples);
}
