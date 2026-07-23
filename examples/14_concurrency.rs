use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..4 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut value = counter.lock().expect("mutex poisoned");
            *value += 1;
        }));
    }

    for handle in handles {
        handle.join().expect("thread failed");
    }

    println!("counter={}", *counter.lock().expect("mutex poisoned"));
}
