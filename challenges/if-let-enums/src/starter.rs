pub enum Message {
    Text(String),
    Number(i32),
    Quit,
    None,
}

pub fn process_message(message: Message) -> String {
    // Your code here...
}

// Example usage
pub fn main() {
    assert_eq!(
        process_message(Message::Text(String::from("Hello"))),
        "Processed Text: Hello"
    );
    assert_eq!(process_message(Message::Number(42)), "Processed Number: 42");
    assert_eq!(process_message(Message::Quit), "Quit Command Received");
    assert_eq!(process_message(Message::None), "No Message");
}
