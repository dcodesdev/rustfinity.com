use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};

#[derive(Clone, Debug)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl Priority {
    fn as_str(&self) -> &'static str {
        match self {
            Priority::Low => "LOW",
            Priority::Medium => "MED",
            Priority::High => "HIGH",
            Priority::Critical => "CRIT",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Message {
    pub content: String,
    pub sender_id: u32,
    pub priority: Priority,
}

pub fn create_producer_thread(messages: Vec<Message>, tx: Sender<Message>) -> JoinHandle<()> {
    thread::spawn(move || {
        for mut msg in messages {
            // Always update priority based on content analysis
            msg.priority = if msg.content.contains("ERROR") {
                Priority::Critical
            } else if msg.content.contains("WARNING") {
                Priority::High
            } else if msg.content.contains("DEBUG") {
                Priority::Medium
            } else {
                Priority::Low
            };

            tx.send(msg).unwrap();
        }
    })
}

pub fn create_consumer_thread(rx: Receiver<Message>) -> JoinHandle<Vec<String>> {
    thread::spawn(move || {
        let mut results = vec![];
        while let Ok(msg) = rx.recv() {
            results.push(format!(
                "[{}|{}] {}",
                msg.priority.as_str(),
                msg.sender_id,
                msg.content
            ));
        }
        results
    })
}

pub fn create_message_channel() -> (Sender<Message>, Receiver<Message>) {
    mpsc::channel()
}

// Example Usage
pub fn main() {
    let (tx, rx) = create_message_channel();

    let mut producer_handles = vec![];
    for id in 0..3 {
        let tx_clone = tx.clone();
        let messages = vec![
            Message {
                content: format!("Normal message from producer {}", id),
                sender_id: id,
                priority: Priority::Low,
            },
            Message {
                content: format!("WARNING: System running hot on producer {}", id),
                sender_id: id,
                priority: Priority::Low,
            },
            Message {
                content: format!("ERROR: Connection lost on producer {}", id),
                sender_id: id,
                priority: Priority::Low,
            },
        ];
        let handle = create_producer_thread(messages, tx_clone);
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
