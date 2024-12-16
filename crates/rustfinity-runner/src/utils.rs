use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};

use base64::{prelude::BASE64_STANDARD, Engine};
use duct::cmd;

pub fn to_utf8(base64: &str) -> anyhow::Result<String> {
    let utf8 = BASE64_STANDARD.decode(base64)?;
    Ok(String::from_utf8(utf8)?)
}

pub async fn run_command_and_merge_output(
    command: &str,
    args: &[String],
    cwd: Option<&str>,
) -> anyhow::Result<String> {
    let cwd = cwd.unwrap_or(".");

    let output = cmd!(command, args.join(" "))
        .stderr_to_stdout()
        .stdout_capture()
        .dir(cwd)
        // don't care about exit code
        .unchecked()
        .run()?;

    Ok(String::from_utf8(output.stdout)?)
}

pub fn write_file(path: &Path, content: &str) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?; // Ensure parent directories exist
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create(true) // Create the file if it doesn't exist
        .truncate(true) // Replace the content if the file exists
        .open(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
