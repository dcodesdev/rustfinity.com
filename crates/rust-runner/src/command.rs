use base64::prelude::*;
use duct::cmd;
use std::fs;
use std::path::Path;

const CHALLENGES_PATH: &str = "../../challenges";

pub async fn run_code(code_base64: &str, challenge: &str) -> anyhow::Result<String> {
    let code_utf8 = BASE64_STANDARD.decode(code_base64)?;

    let code = String::from_utf8(code_utf8)?;

    let repo_path = format!("{}/{}", CHALLENGES_PATH, challenge);
    let repository_path = Path::new(&repo_path).canonicalize()?;

    // write src/lib.rs
    let lib_path = repository_path.join("src/lib.rs");

    fs::write(&lib_path, &code)?;

    let cwd = repository_path
        .to_str()
        .ok_or(anyhow::anyhow!("Invalid path"))?;

    let output = run_command_and_merge_output("cargo", &["test"], cwd).await?;

    Ok(output)
}

pub async fn run_command_and_merge_output(
    command: &str,
    args: &[&str],
    cwd: &str,
) -> anyhow::Result<String> {
    let output = cmd!(command, args.join(" "))
        .stderr_to_stdout()
        .stdout_capture()
        .dir(cwd)
        // don't care about exit code
        .unchecked()
        .run()?;

    Ok(String::from_utf8(output.stdout)?)
}
