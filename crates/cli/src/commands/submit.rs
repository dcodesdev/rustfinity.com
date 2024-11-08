use anyhow::Result;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use std::fs;
use std::path::Path;
use webbrowser;

use crate::dir;

pub async fn submit_challenge() -> Result<()> {
    let slug = dir::get_current_dir()?;

    let lib_path = Path::new("./src/lib.rs");
    let code = fs::read_to_string(lib_path)?;

    let encoded_code = BASE64_STANDARD.encode(&code);

    let url = format!(
        "https://www.rustfinity.com/practice/rust/challenges/{}/description?code={}",
        slug, encoded_code
    );

    webbrowser::open(&url)?;

    Ok(())
}
