use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

// Create and return a new shared counter initialized to 0
pub fn create_counter() -> Arc<Mutex<i32>> {
    // Implement here: return a new Arc<Mutex<i32>> initialized to 0
    unimplemented!()
}

// Increment the shared counter using multiple threads
pub fn increment_counter(counter: Arc<Mutex<i32>>, threads: usize, increments: usize) {
    // Implement here:
    // 1. Create a vector to store thread handles
    // 2. Spawn 'threads' number of threads
    // 3. Each thread should increment the counter 'increments' times
    // 4. Join all threads before returning
    unimplemented!()
}

// Generic function to modify shared data in a new thread
pub fn modify_shared_data<T: Send + 'static>(
    data: Arc<Mutex<T>>,
    modifier: fn(&mut T),
) -> JoinHandle<()> {
    // Implement here:
    // 1. Spawn a new thread
    // 2. Lock the mutex and apply the modifier function
    // 3. Return the JoinHandle
    unimplemented!()
}

// Example usage
pub fn main() {
    // Create a new counter
    let counter = create_counter();

    // Increment it with 5 threads, 10 increments each
    increment_counter(Arc::clone(&counter), 5, 10);
    println!("Counter value: {}", *counter.lock().unwrap());

    // Example of modifying shared string data
    let shared_string = Arc::new(Mutex::new(String::from("Hello")));
    let handle = modify_shared_data(shared_string.clone(), |s| s.push_str(" World"));
    handle.join().unwrap();
    println!("Modified string: {}", *shared_string.lock().unwrap());
}
