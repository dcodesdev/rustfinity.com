use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

pub fn start_channel_system(num_producers: usize, messages_per_producer: usize) -> Vec<String> {
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    let mut handles = vec![];

    // Create producer threads
    for producer_id in 0..num_producers {
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            for msg_num in 0..messages_per_producer {
                let msg = format!("Message from producer {} - {}", producer_id, msg_num);
                tx_clone.send(msg).unwrap();
            }
        });
        handles.push(handle);
    }

    // Drop the original sender to close the channel when producers are done
    drop(tx);

    // Create consumer thread
    let consumer_handle = thread::spawn(move || {
        let mut results = vec![];
        while let Ok(msg) = rx.recv() {
            results.push(format!("Processed: {}", msg));
        }
        results
    });

    // Wait for all producer threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Get the consumer's results
    consumer_handle.join().unwrap()
}
