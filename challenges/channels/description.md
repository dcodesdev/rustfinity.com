Concurrency is an essential concept in modern programming. Rust provides robust tools for concurrent programming, including threads and channels for message passing. Channels enable safe communication between threads using the `std::sync::mpsc` module (where `mpsc` stands for "multiple producer, single consumer").

In this challenge, you will create a producer-consumer system where multiple threads generate messages (producers) and a single thread processes them (consumer). You'll use Rust's channels to send messages safely across threads.

## Your Task

Your task is to implement a function `start_channel_system` that:

1. Creates a channel using `std::sync::mpsc`.
2. Spawns a specified number of producer threads that send messages through the channel.
3. Spawns a single consumer thread that receives and processes messages from the channel.
4. Returns a `Vec<String>` containing all processed messages in the order they were received.

Producers should send unique string messages to the channel, and the consumer should append `"Processed: "` to each message.

### Requirements

- The number of producer threads and the messages each sends are provided as input.
- Messages should be strings of the form `"Message from producer {id} - {msg_number}"`, where `{id}` is the producer ID and `{msg_number}` is the message number.
- The consumer thread should receive the messages and add `"Processed: "` to the beginning of each message.
- The function must return all processed messages in a `Vec<String>` in the order they were received.

## Hints

<details>
    <summary>Click here to reveal hints</summary>

- Use the `std::sync::mpsc::channel` function to create a channel.
- You can clone the sender to allow multiple producers to send messages.
- Use `thread::spawn` to create threads.
- The consumer thread should run a loop to receive messages using `recv()` or `try_recv()` until all producers are finished.
- Consider using `join` on threads to ensure all threads complete before collecting the results.
- Handle channel closing properly by dropping the sender in the main thread when all producers are done.

</details>
