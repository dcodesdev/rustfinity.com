use result_to_option::*;
use std::fs::{remove_file, write};

#[test]
fn test_read_first_line_as_option_valid_file() {
    let file_path = "test_valid.txt";
    write(file_path, "Hello, world!\nSecond line").unwrap();
    assert_eq!(
        read_first_line_as_option(file_path),
        Some("Hello, world!".to_string())
    );
    remove_file(file_path).unwrap();
}

#[test]
fn test_read_first_line_as_option_empty_file() {
    let file_path = "test_empty.txt";
    write(file_path, "").unwrap();
    assert_eq!(read_first_line_as_option(file_path), Some("".to_string()));
    remove_file(file_path).unwrap();
}

#[test]
fn test_read_first_line_as_option_nonexistent_file() {
    let file_path = "nonexistent.txt";
    assert_eq!(read_first_line_as_option(file_path), None);
}

#[test]
fn test_read_first_line_as_option_file_with_only_newline() {
    let file_path = "test_newline.txt";
    write(file_path, "\n").unwrap();
    assert_eq!(read_first_line_as_option(file_path), Some("".to_string()));
    remove_file(file_path).unwrap();
}

#[test]
fn test_read_first_line_as_option_file_with_whitespace() {
    let file_path = "test_whitespace.txt";
    write(file_path, "   \nSecond line").unwrap();
    assert_eq!(read_first_line_as_option(file_path), Some("".to_string()));
    remove_file(file_path).unwrap();
}

#[test]
fn test_read_first_line_as_option_long_first_line() {
    let file_path = "test_long_line.txt";
    write(file_path, "This is a very long first line.\nAnother line.").unwrap();
    assert_eq!(
        read_first_line_as_option(file_path),
        Some("This is a very long first line.".to_string())
    );
    remove_file(file_path).unwrap();
}
