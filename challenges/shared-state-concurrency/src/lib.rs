use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

pub fn create_shared_data<T>(initial: T) -> Arc<Mutex<T>> {
    Arc::new(Mutex::new(initial))
}

pub fn increment_counter(
    counter: Arc<Mutex<i32>>,
    threads: usize,
    increments: usize,
) -> Vec<JoinHandle<()>> {
    let mut handles = vec![];

    for _ in 0..threads {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..increments {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
            }
        });
        handles.push(handle);
    }

    handles
}

pub fn modify_shared_data<T: Send + 'static>(
    data: Arc<Mutex<T>>,
    modifier: fn(&mut T),
) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut data = data.lock().unwrap();
        modifier(&mut data);
    })
}

// Example usage that matches starter.rs
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
