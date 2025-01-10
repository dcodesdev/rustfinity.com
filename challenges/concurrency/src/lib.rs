use std::{ops::Add, thread};

pub fn concurrent_add<T>(items: Vec<T>, num: T) -> Vec<thread::JoinHandle<T>>
where
    T: Send + Copy + Add<Output = T> + 'static,
{
    let mut handles = vec![];

    for item in items.into_iter() {
        let handle = thread::spawn(move || {
            let result = item.add(num);

            result
        });

        handles.push(handle);
    }

    handles
}

pub fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    let handles = concurrent_add(v.clone(), 3);

    for handle in handles {
        let result = handle.join().unwrap();
        let original = v.remove(0);

        assert_eq!(result, original + 3);
    }

    println!("All items have been added successfully!");
}
