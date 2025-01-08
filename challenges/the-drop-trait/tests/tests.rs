use the_drop_trait::TempFile;

#[test]
fn test_tempfile_creation_with_name() {
    let temp_file = TempFile::new("test_temp_file.tmp").expect("Failed to create temporary file");
    assert!(temp_file.path.exists());
}

#[test]
fn test_tempfile_deletion_with_name() {
    let path;
    {
        let temp_file =
            TempFile::new("test_temp_file_to_delete.tmp").expect("Failed to create temporary file");
        path = temp_file.path.clone();
        assert!(path.exists());
    }
    // At this point, the temporary file should have been deleted
    assert!(!path.exists());
}

#[test]
fn test_tempfile_handles_different_name_types() {
    let temp_file_str = TempFile::new("string_name.tmp").expect("Failed to create temporary file");
    assert!(temp_file_str.path.exists());

    let temp_file_string =
        TempFile::new("owned_name.tmp".to_string()).expect("Failed to create temporary file");
    assert!(temp_file_string.path.exists());
}
