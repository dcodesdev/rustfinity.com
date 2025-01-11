use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

pub fn create_counter() -> Arc<Mutex<i32>> {
    Arc::new(Mutex::new(0))
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
