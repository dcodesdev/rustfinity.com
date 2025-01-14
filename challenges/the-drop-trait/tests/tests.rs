use std::path::Path;
use the_drop_trait::TempFile;

#[test]
fn test_tempfile_creation_with_unique_name() {
    let temp_file = TempFile::new("unique_test_file.tmp").expect("Failed to create temporary file");
    assert!(temp_file.path.exists());
    // Ensure the file does not exist after the TempFile is dropped
    drop(temp_file);
    assert!(!Path::new("unique_test_file.tmp").exists());
}

#[test]
fn test_tempfile_creation_with_same_name_overwrites() {
    let file_name = "overwrite_test_file.tmp";

    // Create the first TempFile
    let temp_file_1 = TempFile::new(file_name).expect("Failed to create first temporary file");
    assert!(temp_file_1.path.exists());

    // Create a second TempFile with the same name
    let temp_file_2 = TempFile::new(file_name).expect("Failed to create second temporary file");
    assert!(temp_file_2.path.exists());

    // The second creation overwrites the first one
    assert_eq!(temp_file_1.path, temp_file_2.path);

    // Ensure the file does not exist after both TempFiles are dropped
    drop(temp_file_1);
    drop(temp_file_2);
    assert!(!Path::new(file_name).exists());
}

#[test]
fn test_tempfile_cleanup_on_scope_exit() {
    let path;
    {
        let temp_file =
            TempFile::new("scope_exit_test_file.tmp").expect("Failed to create temporary file");
        path = temp_file.path.clone();
        assert!(path.exists());
    }
    // TempFile should be deleted after leaving the scope
    assert!(!path.exists());
}

#[test]
fn test_tempfile_creation_with_empty_name() {
    let result = TempFile::new("");
    assert!(
        result.is_err(),
        "Expected error when creating a file with an empty name"
    );
}

#[test]
fn test_tempfile_does_not_leave_orphan_files() {
    let file_name = "orphan_test_file.tmp";
    {
        let temp_file = TempFile::new(file_name).expect("Failed to create temporary file");
        assert!(temp_file.path.exists());
    }
    // Ensure the file does not exist after the TempFile goes out of scope
    assert!(!Path::new(file_name).exists());
}
