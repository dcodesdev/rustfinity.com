use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_first_line_as_option(file_path: &str) -> Option<String> {
    let file = File::open(file_path).ok()?; // Open the file, return None on error
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line).ok()?; // Read the first line, return None on error
    Some(line.trim_end().to_string()) // Trim and return the line as Some
}
