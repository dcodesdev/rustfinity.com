pub enum Message {
    Text(String),
    Number(i32),
    Quit,
    None,
}

pub fn process_message(message: Message) -> String {
    // Your code here...
}
