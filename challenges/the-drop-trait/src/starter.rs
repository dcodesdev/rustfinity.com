use std::path::PathBuf;

pub struct TempFile {
    path: PathBuf,
}

impl TempFile {
    // 1. Define the `new` associated function
}

impl Drop for TempFile {
    fn drop(&mut self) {
        // Your code here to delete the file when TempFile is dropped
        unimplemented!()
    }
}

// Example usage
pub fn main() {
    let file_path = PathBuf::from("example_temp_file.tmp");
    let tempfile =
        TempFile::new(file_path.to_str().unwrap()).expect("Failed to create temporary file");

    assert!(tempfile.path.exists(), "File does not exist");

    drop(tempfile);

    assert!(!file_path.exists(), "File was not deleted");

    let tempfile_2 = TempFile::new(&String::from("example_temp_file_2.tmp"))
        .expect("Failed to create temporary file");

    drop(tempfile_2);
}
