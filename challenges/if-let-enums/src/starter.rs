pub enum Message {
    Text(String),
    Number(i32),
    Quit,
    None,
}

pub fn process_text_message(message: &Message) -> String {
    // Your code here...
}

pub fn main() {
    assert_eq!(
        process_text_message(&Message::Text(String::from("Hello"))),
        "Processed Text: Hello"
    );
    assert_eq!(
        process_text_message(&Message::Number(42)),
        "Unhandled Message"
    );
    assert_eq!(process_text_message(&Message::Quit), "Unhandled Message");
    assert_eq!(process_text_message(&Message::None), "Unhandled Message");
}
