use std::fs::File;
use std::io::Read;

pub fn read_file(file_path: &str) -> Option<String> {
    let mut file = File::open(file_path).ok()?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).ok()?;

    Some(contents)
}
