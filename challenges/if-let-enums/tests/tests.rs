use if_let_enums::*;

#[test]
fn test_process_text_message_with_text() {
    let result = process_text_message(&Message::Text(String::from("Hello")));
    assert_eq!(result, "Processed Text: Hello");

    let result = process_text_message(&Message::Text(String::from("Rust is great!")));
    assert_eq!(result, "Processed Text: Rust is great!");
}

#[test]
fn test_process_text_message_with_other_variants() {
    let result = process_text_message(&Message::Number(42));
    assert_eq!(result, "Unhandled Message");

    let result = process_text_message(&Message::Quit);
    assert_eq!(result, "Unhandled Message");

    let result = process_text_message(&Message::None);
    assert_eq!(result, "Unhandled Message");
}

#[test]
fn test_process_text_message_edge_case() {
    let result = process_text_message(&Message::Text(String::from("")));
    assert_eq!(result, "Processed Text: ");
}
