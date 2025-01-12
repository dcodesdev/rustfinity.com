In multi-threaded programming, concurrent execution allows tasks to be performed simultaneously, making programs faster and more efficient. Rust's `std::thread` module facilitates this by enabling the creation and management of threads in a straightforward and safe manner.

In this challenge, you will implement a function `concurrent_add` that spawns threads to add a specified value to each element of a list. Each thread will handle one element of the list, returning the result of the addition. By leveraging threads, you'll distribute the workload across multiple cores for concurrent execution.

## Your Task

Your task is to implement the function `concurrent_add` that takes a list of items (`Vec<T>`) and a number (`T`) as inputs. For each item in the list, the function should spawn a new thread that adds the given number to the item. The function will return a vector of `JoinHandle<T>` representing the handles to all the threads, allowing the results to be retrieved later.

### Requirements

- The function should accept a `Vec<T>` and a value of type `T`.
- `T` can be, integer, floating-point, or any other number type in Rust.
- The function should return a vector of `JoinHandle<T>`.

## Hints

<details>
<summary>Click here to reveal hints</summary>

- The `std::thread::spawn` function creates a new thread to execute a given closure.
- The `std::ops::Add` trait allows addition between two values of the same type.
- In order for a type to be passed to threads safely, they must implement the `Send` trait.
- The type should also have a lifetime of `'static` to make sure the thread can not outlive the data it references.
- Ensure the type `T` implements `Copy` since threads require ownership of their inputs.

</details>
