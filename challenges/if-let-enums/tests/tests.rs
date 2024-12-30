use if_let_enums::*;

#[test]
fn test_process_message_text() {
    let result = process_message(Message::Text(String::from("Hello")));
    assert_eq!(result, "Processed Text: Hello");
}

#[test]
fn test_process_message_number() {
    let result = process_message(Message::Number(42));
    assert_eq!(result, "Processed Number: 42");
}

#[test]
fn test_process_message_quit() {
    let result = process_message(Message::Quit);
    assert_eq!(result, "Quit Command Received");
}

#[test]
fn test_process_message_none() {
    let result = process_message(Message::None);
    assert_eq!(result, "No Message");
}
