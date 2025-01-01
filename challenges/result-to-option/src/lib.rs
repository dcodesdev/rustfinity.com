pub fn read_file(file_path: &str) -> Option<String> {
    std::fs::read_to_string(file_path).ok()
}
