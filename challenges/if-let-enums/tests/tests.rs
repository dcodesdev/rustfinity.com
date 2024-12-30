use if_let_enums::*;

#[test]
fn test_process_text_message_with_text() {
    let result = process_text_message(&Message::Text(String::from("Hello")));
    assert_eq!(result, "Processed Text: Hello");

    let result = process_text_message(&Message::Text(String::from("Rust is great!")));
    assert_eq!(result, "Processed Text: Rust is great!");

    let result = process_text_message(&Message::Text(String::from("Test message with spaces")));
    assert_eq!(result, "Processed Text: Test message with spaces");

    let result = process_text_message(&Message::Text(String::from("Special chars: !@#$%^&*()")));
    assert_eq!(result, "Processed Text: Special chars: !@#$%^&*()");
}

#[test]
fn test_process_text_message_with_other_variants() {
    let result = process_text_message(&Message::Number(42));
    assert_eq!(result, "Unhandled Message");

    let result = process_text_message(&Message::Number(-100));
    assert_eq!(result, "Unhandled Message");

    let result = process_text_message(&Message::Quit);
    assert_eq!(result, "Unhandled Message");

    let result = process_text_message(&Message::None);
    assert_eq!(result, "Unhandled Message");
}

#[test]
fn test_process_text_message_edge_cases() {
    let result = process_text_message(&Message::Text(String::from("")));
    assert_eq!(result, "Processed Text: ");

    let result = process_text_message(&Message::Text(String::from(" ")));
    assert_eq!(result, "Processed Text:  ");

    let result = process_text_message(&Message::Text(String::from("\n")));
    assert_eq!(result, "Processed Text: \n");
}

#[test]
fn test_process_text_message_with_mixed_messages() {
    let text1 = Message::Text(String::from("First message"));
    let text2 = Message::Text(String::from("Second message"));
    let messages = vec![
        &text1,
        &Message::Number(123),
        &Message::Quit,
        &text2,
        &Message::None,
    ];

    let expected = vec![
        "Processed Text: First message",
        "Unhandled Message",
        "Unhandled Message",
        "Processed Text: Second message",
        "Unhandled Message",
    ];

    for (msg, exp) in messages.into_iter().zip(expected.into_iter()) {
        assert_eq!(process_text_message(msg), exp);
    }
}
