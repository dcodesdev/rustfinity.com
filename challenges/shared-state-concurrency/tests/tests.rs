use shared_state_concurrency::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[test]
fn test_create_counter() {
    let counter = create_counter();
    assert_eq!(
        *counter.lock().unwrap(),
        0,
        "New counter should be initialized to zero"
    );
}

#[test]
fn test_increment_counter() {
    let counter = create_counter();
    let handles = increment_counter(Arc::clone(&counter), 5, 10);

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(
        *counter.lock().unwrap(),
        50,
        "Counter should equal threads * increments (5 * 10 = 50)"
    );
}

#[test]
fn test_modify_shared_data() {
    let data = Arc::new(Mutex::new(String::from("hello")));
    let handle = modify_shared_data(Arc::clone(&data), |s| s.push_str(" world"));
    handle.join().unwrap();
    assert_eq!(
        *data.lock().unwrap(),
        "hello world",
        "String should be modified correctly"
    );
}

#[test]
fn test_multiple_modifications() {
    let counter = create_counter();

    let handles1 = increment_counter(Arc::clone(&counter), 3, 5);
    for handle in handles1 {
        handle.join().unwrap();
    }
    assert_eq!(
        *counter.lock().unwrap(),
        15,
        "First modification should result in 3 * 5 = 15"
    );

    let handles2 = increment_counter(Arc::clone(&counter), 2, 3);
    for handle in handles2 {
        handle.join().unwrap();
    }
    assert_eq!(
        *counter.lock().unwrap(),
        21,
        "After second modification should be 15 + (2 * 3) = 21"
    );
}

#[test]
fn test_concurrent_modifications() {
    let counter = create_counter();
    let counter2 = create_counter();

    let handles1 = increment_counter(Arc::clone(&counter), 2, 5);
    let handles2 = increment_counter(Arc::clone(&counter), 3, 3);

    for handle in handles1.into_iter().chain(handles2) {
        handle.join().unwrap();
    }

    assert_eq!(
        *counter.lock().unwrap(),
        19,
        "Combined concurrent modifications should be (2 * 5) + (3 * 3) = 19"
    );
    assert_eq!(
        *counter2.lock().unwrap(),
        0,
        "Unmodified counter should remain at 0"
    );
}

#[test]
fn test_zero_increments() {
    let counter = create_counter();
    let handles = increment_counter(Arc::clone(&counter), 5, 0);

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(
        *counter.lock().unwrap(),
        0,
        "Counter should remain 0 when increment count is 0"
    );
}

#[test]
fn test_modify_with_delay() {
    let data = Arc::new(Mutex::new(0));

    let handle = modify_shared_data(Arc::clone(&data), |n| {
        thread::sleep(Duration::from_millis(50));
        *n += 1;
    });

    assert_eq!(
        *data.lock().unwrap(),
        0,
        "Value should be 0 before modification completes"
    );

    handle.join().unwrap();
    assert_eq!(
        *data.lock().unwrap(),
        1,
        "Value should be 1 after modification completes"
    );
}

#[test]
fn test_multiple_counters() {
    let counter1 = create_counter();
    let counter2 = create_counter();

    let handles1 = increment_counter(Arc::clone(&counter1), 2, 3);
    let handles2 = increment_counter(Arc::clone(&counter2), 3, 2);

    for handle in handles1 {
        handle.join().unwrap();
    }
    for handle in handles2 {
        handle.join().unwrap();
    }

    assert_eq!(
        *counter1.lock().unwrap(),
        6,
        "First counter should be 2 threads * 3 increments = 6"
    );
    assert_eq!(
        *counter2.lock().unwrap(),
        6,
        "Second counter should be 3 threads * 2 increments = 6"
    );
}

#[test]
fn test_num_of_threads() {
    let counter = create_counter();
    let expected_threads = 5;
    let handles = increment_counter(Arc::clone(&counter), expected_threads, 1);

    assert_eq!(
        handles.len(),
        expected_threads,
        "Should spawn exactly {} threads",
        expected_threads
    );

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(
        *counter.lock().unwrap(),
        expected_threads as i32,
        "Final count should match number of threads when each increments once"
    );
}

#[test]
fn test_increments() {
    let counter = create_counter();
    let expected_increments = 10;
    let handles = increment_counter(Arc::clone(&counter), 1, expected_increments);

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(
        *counter.lock().unwrap(),
        expected_increments as i32,
        "Single thread should increment exactly {} times",
        expected_increments
    );
}
