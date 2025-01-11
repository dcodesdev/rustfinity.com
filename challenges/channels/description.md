Concurrency is an essential concept in modern programming. This challenge focuses on building a message processing system using Rust's channels and threads. You'll create a system where multiple producers send messages with different priority levels, and a single consumer processes and formats these messages.

## Your Task

Implement three key functions that work with a message processing system:

1. `create_message_channel()` - Creates a channel for sending Message structs
2. `create_producer_thread()` - Creates a thread that analyzes and forwards messages
3. `create_consumer_thread()` - Creates a thread that formats and collects messages

### Requirements

#### Producer Thread

- Receives a vector of messages and a sender channel
- Updates priority based on content keywords:
  - "ERROR" → Critical
  - "WARNING" → High
  - "DEBUG" → Medium
  - Others become Low
- Sends each updated message through the channel

#### Consumer Thread

- Receives messages until the channel is closed
- Formats each message as: `[PRIORITY|SENDER_ID] CONTENT`
  where PRIORITY is one of: LOW, MED, HIGH, CRIT
- Returns a vector of formatted message strings

### Notes

- Ignore error handling, you can simply `unwrap()`
- Have a look at the `main` function to see how the functions are used.

## Hints

Here are some tips to help you get started:

<details>
    <summary>Click here for implementation hints</summary>

- Use `mpsc::channel()` to create the message channel
- Use `thread::spawn` and move closures
- Use `while let Ok(msg) = rx.recv()` for receiving
- Format with `format!("[{}|{}] {}", priority, id, content)`

</details>
