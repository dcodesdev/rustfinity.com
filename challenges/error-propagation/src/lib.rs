use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn sum_integers_from_file(file_path: &str) -> Result<i32, String> {
    let file = File::open(file_path).map_err(|e| format!("Error reading file: {}", e))?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines() {
        let line = line.map_err(|e| format!("Error reading line: {}", e))?;
        let number: i32 = line
            .trim()
            .parse()
            .map_err(|_| format!("Invalid integer in file: {}", line))?;
        sum += number;
    }

    Ok(sum)
}
