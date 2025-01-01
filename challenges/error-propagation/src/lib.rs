use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn sum_integers_from_file(file_path: &str) -> Result<i32, io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines() {
        let line = line?;
        let number: i32 = line.trim().parse().map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid integer in file: {}", line),
            )
        })?;
        sum += number;
    }

    Ok(sum)
}
