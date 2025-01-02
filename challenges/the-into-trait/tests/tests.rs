use the_into_trait::print_message;

#[test]
fn test_with_str() {
    print_message("This is a &str.");
}

#[test]
fn test_with_string() {
    let message = String::from("This is a String.");
    print_message(message);
}

#[test]
fn test_with_empty_string() {
    print_message("");
}

#[test]
fn test_with_special_characters() {
    print_message("Special characters: !@#$%^&*()");
}
