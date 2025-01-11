Concurrency is a crucial concept in modern programming, and Rust provides powerful tools to manage it safely. One common challenge in concurrency is managing shared state across multiple threads. Rust's ownership model, along with types like `Mutex` and `Arc` (atomic reference counting), makes this task safe and efficient.

In this challenge, you will implement a generic shared state system and use it to create thread-safe counters and other shared data structures.

## Your Task

Implement three functions that demonstrate different aspects of shared state concurrency:

1. `create_shared_data<T>(initial: T) -> Arc<Mutex<T>>`: Creates shared state with any type T
2. `increment_counter(counter: Arc<Mutex<i32>>, threads: usize, increments: usize) -> Vec<JoinHandle<()>>`: Demonstrates multiple threads modifying shared state
3. `modify_shared_data<T: Send + 'static>(data: Arc<Mutex<T>>, modifier: fn(&mut T)) -> JoinHandle<()>`: Generic function for thread-safe modifications

### Requirements

1. The `create_shared_data` function should:

   - Accept any type T
   - Wrap it in a Mutex and Arc
   - Return the thread-safe container

2. The `increment_counter` function should:

   - Accept an Arc<Mutex<i32>>
   - Spawn the specified number of threads
   - Return handles to all spawned threads

3. The `modify_shared_data` function should:
   - Accept any Send + 'static type
   - Perform modifications in a new thread
   - Return the thread handle

### Example Usage

```rust
// Create shared counter
let counter = create_shared_data(0);

// Increment it with multiple threads
let handles = increment_counter(counter.clone(), 5, 10);
for handle in handles {
    handle.join().unwrap();
}

// Create and modify shared string
let text = create_shared_data(String::from("Hello"));
let handle = modify_shared_data(text.clone(), |s| s.push_str(" World"));
handle.join().unwrap();
```

## Hints

<details>
    <summary>Click here to reveal hints</summary>

- Use `Arc::new(Mutex::new(initial))` to create thread-safe containers
- Remember to clone the Arc before moving it into each new thread
- Use `lock().unwrap()` to access the data inside the Mutex
- The 'static lifetime is required for thread safety

</details>
