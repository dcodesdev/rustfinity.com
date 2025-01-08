use std::path::PathBuf;

pub struct TempFile {
    path: PathBuf,
}

impl TempFile {
    /// Creates a new temporary file with the specified name.
    ///
    /// # Arguments
    /// - `file_name`: The name of the file (accepts both `String` and `&str`).
    ///
    /// # Returns
    /// - `Ok(Self)` if the file is created successfully.
    /// - `Err(std::io::Error)` if an error occurs during file creation.
    pub fn new<S: AsRef<str>>(file_name: S) -> Result<Self, std::io::Error> {
        // Your code here to create the temporary file
        unimplemented!()
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        // Your code here to delete the file when TempFile is dropped
        unimplemented!()
    }
}

// Example usage
pub fn main() {
    let temp_file =
        TempFile::new("example_temp_file.tmp").expect("Failed to create temporary file");
    println!("Temporary file created at: {:?}", temp_file.path);

    // TempFile will automatically delete the file when it goes out of scope.
}
