use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};

// 1. Implement this function to create and return a message channel
pub fn create_message_channel()

pub fn create_producer_thread(
    producer_id: usize,
    messages_count: usize,
    tx: Sender<String>,
) -> JoinHandle<()> {
    // 2. Implement this function to create and return a producer thread
}

pub fn create_consumer_thread(rx: Receiver<String>) -> JoinHandle<Vec<String>> {
    // 3. Implement this function to create and return a consumer thread
}

// Example Usage
pub fn main() {
    // Create a channel using our function
    let (tx, rx) = create_message_channel();

    // Create 3 producer threads
    let mut producer_handles = vec![];
    for id in 0..3 {
        let tx_clone = tx.clone();
        let handle = create_producer_thread(id, 2, tx_clone);
        producer_handles.push(handle);
    }

    // Drop the original sender
    drop(tx);

    // Create the consumer thread
    let consumer_handle = create_consumer_thread(rx);

    // Wait for all producers to finish
    for handle in producer_handles {
        handle.join().unwrap();
    }

    // Get and print the results from consumer
    let results = consumer_handle.join().unwrap();
    println!("Processed messages:");
    for msg in results {
        println!("{}", msg);
    }
}
