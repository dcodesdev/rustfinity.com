use channels::*;

#[test]
fn test_single_producer() {
    let (tx, rx) = create_message_channel();
    let messages = vec![Message {
        content: "Normal message".to_string(),
        sender_id: 0,
        priority: Priority::Low,
    }];

    let handle = create_producer_thread(messages, tx);
    handle.join().unwrap();

    let consumer_handle = create_consumer_thread(rx);
    let results = consumer_handle.join().unwrap();

    assert_eq!(results.len(), 1, "Should process exactly one message");
    assert!(
        results[0].contains("LOW") && results[0].contains("Normal message"),
        "Message should contain priority LOW and the original content"
    );
}

#[test]
fn test_priority_updates() {
    let (tx, rx) = create_message_channel();
    let messages = vec![
        Message {
            content: "Normal message".to_string(),
            sender_id: 1,
            priority: Priority::Low,
        },
        Message {
            content: "WARNING: Test warning".to_string(),
            sender_id: 1,
            priority: Priority::Low,
        },
        Message {
            content: "ERROR: Test error".to_string(),
            sender_id: 1,
            priority: Priority::Low,
        },
    ];

    let handle = create_producer_thread(messages, tx);
    handle.join().unwrap();

    let consumer_handle = create_consumer_thread(rx);
    let results = consumer_handle.join().unwrap();

    assert_eq!(results.len(), 3, "Should process all messages");
    assert!(
        results[0].contains("LOW") && results[0].contains("Normal message"),
        "Normal message should contain LOW priority"
    );
    assert!(
        results[1].contains("HIGH") && results[1].contains("WARNING"),
        "Warning message should contain HIGH priority"
    );
    assert!(
        results[2].contains("CRIT") && results[2].contains("ERROR"),
        "Error message should contain CRITICAL priority"
    );
}

#[test]
fn test_multiple_producers() {
    let (tx, rx) = create_message_channel();

    let mut handles = vec![];
    for id in 0..3 {
        let tx_clone = tx.clone();
        let messages = vec![Message {
            content: format!("Message from {}", id),
            sender_id: id,
            priority: Priority::Low,
        }];
        handles.push(create_producer_thread(messages, tx_clone));
    }
    drop(tx);

    for handle in handles {
        handle.join().unwrap();
    }

    let consumer_handle = create_consumer_thread(rx);
    let results = consumer_handle.join().unwrap();

    assert_eq!(
        results.len(),
        3,
        "Should process one message from each producer"
    );
    assert!(
        results.iter().any(|msg| msg.contains("|0]")),
        "Should have message from producer 0"
    );
    assert!(
        results.iter().any(|msg| msg.contains("|1]")),
        "Should have message from producer 1"
    );
    assert!(
        results.iter().any(|msg| msg.contains("|2]")),
        "Should have message from producer 2"
    );
}

#[test]
fn test_empty_channel() {
    let (tx, rx) = create_message_channel();
    drop(tx);

    let consumer_handle = create_consumer_thread(rx);
    let results = consumer_handle.join().unwrap();
    assert!(
        results.is_empty(),
        "Should return empty vector when no messages are sent"
    );
}

#[test]
fn test_message_format() {
    let (tx, rx) = create_message_channel();
    let messages = vec![Message {
        content: "Test message".to_string(),
        sender_id: 42,
        priority: Priority::Medium, // This priority will be overridden
    }];

    let producer_handle = create_producer_thread(messages, tx);
    producer_handle.join().unwrap();

    let consumer_handle = create_consumer_thread(rx);
    let results = consumer_handle.join().unwrap();

    assert_eq!(results.len(), 1);
    assert!(
        results[0].contains("LOW")  // Changed from MED to LOW since content has no keywords
            && results[0].contains("42")
            && results[0].contains("Test message"),
        "Message should contain correct priority based on content, ID and content"
    );
}
