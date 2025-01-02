use std::{fs, process};
use the_as_ref_trait::print_message;

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

#[test]
fn test_with_whitespace_only() {
    print_message("    ");
}

#[test]
fn test_with_numeric_string() {
    print_message("1234567890");
}

#[test]
fn test_with_unicode_characters() {
    print_message("こんにちは、世界! 🌏"); // "Hello, World!" in Japanese
}

#[test]
fn test_with_long_string() {
    let long_string = "Rust".repeat(1000); // Creates a very long string
    print_message(long_string);
}

#[test]
fn test_with_escape_sequences() {
    print_message("Line 1\nLine 2\tIndented");
}

#[test]
fn test_with_multiline_string() {
    let multiline = r#"This is
a multi-line
string."#;
    print_message(multiline);
}

fn create_bin_and_run(code: &str) -> process::Output {
    let bin = "src/main.rs";
    fs::write(bin, code).unwrap();
    let output = process::Command::new("cargo")
        .arg("run")
        .output()
        .expect("Failed to compile");
    fs::remove_file(bin).unwrap();
    output
}

#[test]
fn test_should_print_the_right_message() {
    let code = r#"
        use the_as_ref_trait::print_message;

        fn main() {
            print_message("Hello, world!");
        }
    "#;

    let output = create_bin_and_run(code);

    let output_str = String::from_utf8(output.stdout).unwrap();

    assert_eq!(output_str, "Hello, world!\n");
}
