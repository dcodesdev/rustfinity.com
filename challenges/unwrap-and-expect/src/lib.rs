use std::env;
use std::fs;

pub fn read_file_to_string(path: &str) -> String {
    fs::read_to_string(path).expect(&format!("Failed to read file: {}", path))
}

pub fn get_env_variable(key: &str) -> String {
    env::var(key).unwrap()
}
