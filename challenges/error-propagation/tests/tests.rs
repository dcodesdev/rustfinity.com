use error_propagation::*;
use std::fs::{remove_file, write};
use std::io;

#[test]
fn test_sum_integers_from_file_valid() {
    let file_path = "test_valid.txt";
    write(file_path, "10\n20\n30\n").unwrap();
    assert_eq!(sum_integers_from_file(file_path).unwrap(), 60);
    remove_file(file_path).unwrap();
}

#[test]
fn test_sum_integers_from_file_empty_file() {
    let file_path = "test_empty.txt";
    write(file_path, "").unwrap();
    assert_eq!(sum_integers_from_file(file_path).unwrap(), 0);
    remove_file(file_path).unwrap();
}

#[test]
fn test_sum_integers_from_file_invalid_data() {
    let file_path = "test_invalid.txt";
    write(file_path, "10\nabc\n30\n").unwrap();
    let result = sum_integers_from_file(file_path);
    assert!(matches!(
        result,
        Err(ref e) if e.kind() == io::ErrorKind::InvalidData
    ));
    remove_file(file_path).unwrap();
}

#[test]
fn test_sum_integers_from_file_file_error() {
    let file_path = "non_existent_file.txt";
    let result = sum_integers_from_file(file_path);
    assert!(result.is_err());
}

#[test]
fn test_sum_integers_from_file_negative_numbers() {
    let file_path = "test_negative.txt";
    write(file_path, "-10\n-20\n30\n").unwrap();
    assert_eq!(sum_integers_from_file(file_path).unwrap(), 0);
    remove_file(file_path).unwrap();
}

#[test]
fn test_sum_integers_from_file_large_numbers() {
    let file_path = "test_large.txt";
    write(file_path, "1000000\n2000000\n").unwrap();
    assert_eq!(sum_integers_from_file(file_path).unwrap(), 3000000);
    remove_file(file_path).unwrap();
}

#[test]
fn test_sum_integers_from_file_single_number() {
    let file_path = "test_single_number.txt";
    write(file_path, "42\n").unwrap();
    assert_eq!(sum_integers_from_file(file_path).unwrap(), 42);
    remove_file(file_path).unwrap();
}
