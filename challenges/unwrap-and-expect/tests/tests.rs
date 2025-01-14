use std::env;
use std::fs;
use unwrap_and_expect::*;

#[test]
fn test_read_file_to_string_success() {
    let path = "test_file.txt";
    fs::write(path, "Test content").expect("Failed to create test file");
    let content = read_file_to_string(path);
    assert_eq!(content, "Test content");
    fs::remove_file(path).expect("Failed to remove test file");
}

#[test]
#[should_panic(expected = "Failed to read file: nonexistent.txt")]
fn test_read_file_to_string_panic() {
    read_file_to_string("nonexistent.txt");
}

#[test]
fn test_read_file_to_string_empty_file() {
    let path = "empty_file.txt";
    fs::write(path, "").expect("Failed to create test file");
    let content = read_file_to_string(path);
    assert_eq!(content, "");
    fs::remove_file(path).expect("Failed to remove test file");
}

#[test]
fn test_read_file_to_string_special_characters() {
    let path = "special_chars_file.txt";
    let content = "Line1\nLine2\tSpecial!@#$%^&*()";
    fs::write(path, content).expect("Failed to create test file");
    let result = read_file_to_string(path);
    assert_eq!(result, content);
    fs::remove_file(path).expect("Failed to remove test file");
}

#[test]
fn test_get_env_variable_success() {
    env::set_var("TEST_ENV_KEY", "TestValue");
    let value = get_env_variable("TEST_ENV_KEY");
    assert_eq!(value, "TestValue");
    env::remove_var("TEST_ENV_KEY");
}

#[test]
#[should_panic]
fn test_get_env_variable_panic() {
    get_env_variable("MISSING_ENV_KEY");
}

#[test]
fn test_get_env_variable_empty_value() {
    env::set_var("EMPTY_ENV_KEY", "");
    let value = get_env_variable("EMPTY_ENV_KEY");
    assert_eq!(value, "");
    env::remove_var("EMPTY_ENV_KEY");
}

#[test]
fn test_get_env_variable_special_characters() {
    let key = "SPECIAL_KEY";
    let value = "Special!@#$%^&*()";
    env::set_var(key, value);
    let result = get_env_variable(key);
    assert_eq!(result, value);
    env::remove_var(key);
}

#[test]
#[should_panic]
fn test_read_file_to_string_after_deletion() {
    let path = "delete_test.txt";
    fs::write(path, "Temporary content").expect("Failed to create test file");
    fs::remove_file(path).expect("Failed to remove test file");
    read_file_to_string(path); // This should panic
}

#[test]
#[should_panic]
fn test_get_env_variable_removed_after_set() {
    let key = "TEMP_ENV_KEY";
    env::set_var(key, "TemporaryValue");
    env::remove_var(key);
    get_env_variable(key); // This should panic
}
