use error_propagation::*;
use std::fs::write;

#[test]
fn test_sum_integers_from_file_valid() {
    let file_path = "test_valid.txt";
    write(file_path, "10\n20\n30\n").unwrap();
    assert_eq!(sum_integers_from_file(file_path).unwrap(), 60);
}

#[test]
fn test_sum_integers_from_file_invalid_data() {
    let file_path = "test_invalid.txt";
    write(file_path, "10\nabc\n30\n").unwrap();
    let result = sum_integers_from_file(file_path);
    assert_eq!(result, Err("Invalid integer in file: abc".to_string()));
}

#[test]
fn test_sum_integers_from_file_file_error() {
    let file_path = "non_existent_file.txt";
    let result = sum_integers_from_file(file_path);
    assert!(result.is_err());
}
