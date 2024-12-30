pub enum Message {
    Text(String),
    Number(i32),
    Quit,
    None,
}

pub fn process_text_message(message: &Message) -> String {
    if let Message::Text(content) = message {
        return format!("Processed Text: {}", content);
    }
    "Unhandled Message".to_string()
}
