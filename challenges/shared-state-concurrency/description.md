Concurrency is a crucial concept in modern programming, and Rust provides powerful tools to manage it safely. One common challenge in concurrency is managing shared state across multiple threads. Rust's ownership model, along with types like `Mutex` and `Arc` (atomic reference counting), makes this task safe and efficient.

In this challenge, you will implement a shared counter that can be safely incremented by multiple threads. The goal is to demonstrate your understanding of Rust's thread and synchronization primitives.

## Your Task

You are tasked with creating a program that spawns multiple threads to increment a shared counter. Each thread should increment the counter a certain number of times, and the program should ensure all increments are safely synchronized to avoid data races.

### Requirements

1. Use `std::sync::Mutex` to protect access to the shared counter.
2. Use `std::sync::Arc` to share the `Mutex` across threads.
3. Spawn a fixed number of threads (e.g., 5), where each thread increments the counter 10 times.
4. After all threads complete, print the final value of the counter.

### Constraints

- Ensure that no data races or unsafe operations occur.
- Properly join all threads before calculating the final value.
- Use Rust's concurrency primitives (`Arc`, `Mutex`, and `thread`).

## Hints

<details>
    <summary>Click here to reveal hints</summary>

- You can use `Arc` (Atomic Reference Counted) to allow multiple threads to access the same `Mutex`-protected counter.
- Use `thread::spawn` to create threads and `.join()` to wait for them to complete.
- To access and modify the counter, lock the `Mutex` using the `.lock()` method. This will return a `Result` which you can safely unwrap in this case.
- Remember to keep your main thread alive until all worker threads are finished.

</details>
