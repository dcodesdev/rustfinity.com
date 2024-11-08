use anyhow::Result;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use std::env;
use std::fs;
use std::path::Path;
use webbrowser;

pub async fn submit_challenge() -> Result<()> {
    // Get the current directory name as slug
    let current_dir = env::current_dir()?;
    let slug = current_dir
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| anyhow::anyhow!("Failed to get directory name"))?;

    // Read the code from ./src/lib.rs
    let lib_path = Path::new("./src/lib.rs");
    let code = fs::read_to_string(lib_path)?;

    // Encode the code in base64
    let encoded_code = BASE64_STANDARD.encode(&code);

    // Construct the URL
    let url = format!(
        "https://www.rustfinity.com/practice/rust/challenges/{}/description?code={}",
        slug, encoded_code
    );

    // Open the user's browser to the URL
    webbrowser::open(&url)?;

    Ok(())
}
