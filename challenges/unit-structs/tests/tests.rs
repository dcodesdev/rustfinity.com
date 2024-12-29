use std::{fs, process};
use unit_structs::Logger;

fn create_bin_and_run(code: &str) -> process::Output {
    let bin = "src/main.rs";
    fs::write(bin, code).unwrap();
    let output = process::Command::new("cargo")
        .arg("run")
        .output()
        .expect("Failed to compile");
    fs::remove_file(bin).unwrap();
    output
}

#[test]
fn test_compiles() {
    Logger::log_message("Test message");
}

#[test]
fn test_log_message() {
    let code = r#"
        use unit_structs::Logger;
        pub fn main() {
            Logger::log_message("Test message");
            Logger::log_message("Rust is great");
            Logger::log_message("I love Rust");
            Logger::log_message("Rust is the best");
        }
    "#;

    let output = create_bin_and_run(code);

    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.contains("Test message"));
    assert!(stdout.contains("Rust is great"));
    assert!(stdout.contains("I love Rust"));
    assert!(stdout.contains("Rust is the best"));
}
