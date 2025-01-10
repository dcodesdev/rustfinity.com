use std::thread;

pub fn concurrent_add<T>(items: Vec<T>) -> Vec<thread::JoinHandle<()>> {
    // Implement the function here
}

// Example Usage
pub fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let handles = concurrent_logging(v);

    for handle in handles {
        handle.join().unwrap();
    }

    // Must log all items, each in a different thread
}
