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

// Example Usage
pub fn main() {
    {
        let mut list = vec![1, 2, 3, 4, 5];

        let handles = concurrent_add(list.clone(), 3);

        for handle in handles {
            let result = handle.join().unwrap();
            let original = list.remove(0);

            assert_eq!(result, original + 3);
        }
    }

    {
        let mut list = vec![10., 20., 30., 40., 50.];

        let handles = concurrent_add(list.clone(), 5.);

        for handle in handles {
            let result = handle.join().unwrap();
            let original = list.remove(0);

            assert_eq!(result, original + 5.);
        }
    }
}
