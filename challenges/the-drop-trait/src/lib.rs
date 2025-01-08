use std::env;
use std::fs::{self, File};
use std::io;
use std::path::PathBuf;

pub struct TempFile {
    pub path: PathBuf,
}

impl TempFile {
    pub fn new<S: AsRef<str>>(file_name: S) -> Result<Self, io::Error> {
        let mut temp_path = env::temp_dir();
        temp_path.push(file_name.as_ref());

        File::create(&temp_path)?; // Create the file
        Ok(TempFile { path: temp_path })
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        if let Err(e) = fs::remove_file(&self.path) {
            eprintln!("Failed to delete temporary file: {}", e);
        }
    }
}
