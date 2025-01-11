use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

pub fn create_shared_data<T>(initial: T) -> Arc<Mutex<T>> {
    // 1. Initialize and return a new Arc<Mutex<T>> with the initial value
}

pub fn increment_counter(
    counter: Arc<Mutex<i32>>,
    threads: usize,
    increments: usize,
) -> Vec<JoinHandle<()>> {
    // 2. Increment the counter by the given increments using the given number of threads
}

pub fn modify_shared_data<T: Send + 'static>(
    data: Arc<Mutex<T>>,
    modifier: fn(&mut T),
) -> JoinHandle<()> {
    // 3. Use a new thread to modify the shared data
}

// Example usage
pub fn main() {
    let counter = create_shared_data(0);
    let handles = increment_counter(Arc::clone(&counter), 5, 10);
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Counter value: {}", *counter.lock().unwrap());

    let shared_string = create_shared_data(String::from("Hello"));
    let handle = modify_shared_data(shared_string.clone(), |s| s.push_str(" World"));
    handle.join().unwrap();
    println!("Modified string: {}", *shared_string.lock().unwrap());
}
