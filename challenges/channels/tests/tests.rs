use channels::*;
use std::sync::mpsc;

#[test]
fn test_single_producer() {
    let (tx, rx) = mpsc::channel();
    let handle = create_producer_thread(0, 1, tx);
    handle.join().unwrap();

    let messages: Vec<String> = rx.iter().collect();
    assert_eq!(
        messages.len(),
        1,
        "Single producer should generate exactly one message"
    );
    assert_eq!(
        messages[0], "Message from producer 0 - 0",
        "Message format should match 'Message from producer 0 - 0'"
    );
}

#[test]
fn test_multiple_messages_from_producer() {
    let (tx, rx) = mpsc::channel();
    let handle = create_producer_thread(1, 3, tx);

    handle.join().unwrap();

    let messages: Vec<String> = rx.iter().collect();

    assert_eq!(
        messages.len(),
        3,
        "Producer should generate exactly three messages"
    );
    assert!(
        messages.contains(&"Message from producer 1 - 0".to_string()),
        "Should contain message with number 0"
    );
    assert!(
        messages.contains(&"Message from producer 1 - 1".to_string()),
        "Should contain message with number 1"
    );
    assert!(
        messages.contains(&"Message from producer 1 - 2".to_string()),
        "Should contain message with number 2"
    );
}

#[test]
fn test_consumer_basic_processing() {
    let (tx, rx) = mpsc::channel();
    let consumer_handle = create_consumer_thread(rx);

    tx.send("Test message".to_string()).unwrap();
    drop(tx);

    let results = consumer_handle.join().unwrap();
    assert_eq!(
        results.len(),
        1,
        "Consumer should process exactly one message"
    );
    assert_eq!(
        results[0], "Processed: Test message",
        "Consumer should prepend 'Processed: ' to the message"
    );
}

#[test]
fn test_consumer_multiple_messages() {
    let (tx, rx) = mpsc::channel();
    let consumer_handle = create_consumer_thread(rx);

    tx.send("First".to_string()).unwrap();
    tx.send("Second".to_string()).unwrap();
    tx.send("Third".to_string()).unwrap();
    drop(tx);

    let results = consumer_handle.join().unwrap();
    assert_eq!(
        results.len(),
        3,
        "Consumer should process exactly three messages"
    );
    assert_eq!(
        results[0], "Processed: First",
        "First message should be processed correctly"
    );
    assert_eq!(
        results[1], "Processed: Second",
        "Second message should be processed correctly"
    );
    assert_eq!(
        results[2], "Processed: Third",
        "Third message should be processed correctly"
    );
}

#[test]
fn test_consumer_empty_channel() {
    let (tx, rx) = mpsc::channel();
    let consumer_handle = create_consumer_thread(rx);
    drop(tx);

    let results = consumer_handle.join().unwrap();
    assert!(
        results.is_empty(),
        "Consumer should return empty vector when no messages are sent"
    );
}

#[test]
fn test_producer_consumer_integration() {
    let (tx, rx) = mpsc::channel();
    let producer_handle = create_producer_thread(42, 1, tx.clone());
    drop(tx);
    let consumer_handle = create_consumer_thread(rx);

    producer_handle.join().unwrap();
    let results = consumer_handle.join().unwrap();

    assert_eq!(
        results.len(),
        1,
        "Integration test should process exactly one message"
    );
    assert_eq!(
        results[0], "Processed: Message from producer 42 - 0",
        "Producer and consumer should work together correctly"
    );
}

#[test]
fn test_message_format_correctness() {
    let (tx, rx) = mpsc::channel();
    let handle = create_producer_thread(99, 1, tx);
    handle.join().unwrap();

    let messages: Vec<String> = rx.iter().collect();
    assert!(
        !messages.is_empty(),
        "Producer should generate at least one message"
    );
    assert!(
        messages[0].contains("producer 99"),
        "Message should contain correct producer ID"
    );
    assert!(
        messages[0].contains("- 0"),
        "Message should contain correct message number"
    );
}
