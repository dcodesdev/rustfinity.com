pub enum Message {
    Text(String),
    Number(i32),
    Quit,
    None,
}

pub fn process_message(message: Message) -> String {
    if let Message::Text(content) = message {
        return format!("Processed Text: {}", content);
    } else if let Message::Number(value) = message {
        return format!("Processed Number: {}", value);
    } else if let Message::Quit = message {
        return "Quit Command Received".to_string();
    } else if let Message::None = message {
        return "No Message".to_string();
    }

    unreachable!() // This should never be reached since all variants are handled.
}
