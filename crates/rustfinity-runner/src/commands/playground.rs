use std::fs;
use std::path::Path;

use crate::utils::{run_command_and_merge_output, to_utf8};

pub struct PlaygroundParams {
    code_base64: String,
}

impl PlaygroundParams {
    pub fn new(code_base64: String) -> Self {
        Self { code_base64 }
    }
}

pub async fn run_code_in_playground(params: &PlaygroundParams) -> anyhow::Result<String> {
    let PlaygroundParams { code_base64 } = params;

    let mut output = String::new();

    let tests_output = execute_code(&code_base64).await?;
    output.push_str(&tests_output);

    Ok(output)
}

async fn execute_code(code_base64: &str) -> anyhow::Result<String> {
    let code = to_utf8(code_base64)?;

    let cwd = "/app/playground";
    let main_path = Path::new(cwd).join("src/main.rs");

    // Write src/main.rs
    fs::write(&main_path, &code)?;
    let output = run_command_and_merge_output("cargo", &["run"], Some(cwd)).await?;

    Ok(output)
}
