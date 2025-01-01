use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_first_line_as_option(file_path: &str) -> Option<String> {
    let file = File::open(file_path).ok()?;

    let mut reader = BufReader::new(file);
    let mut line = String::new();

    reader.read_line(&mut line).ok()?;

    Some(line.trim_end().to_string())
}
