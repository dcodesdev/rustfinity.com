use base64::prelude::*;
use duct::cmd;
use std::env;
use std::fs;
use std::path::Path;

pub async fn run_code(code_base64: &str, challenge: &str) -> anyhow::Result<String> {
    let code_utf8 = BASE64_STANDARD.decode(code_base64)?;
    let code = String::from_utf8(code_utf8)?;

    if challenge == "playground" {
        let cwd = "/app/playground";
        let main_path = Path::new(cwd).join("src/main.rs");

        // Write src/main.rs
        fs::write(&main_path, &code)?;

        // Run the code
        let output = run_command_and_merge_output("cargo", &["run"], cwd).await?;

        return Ok(output);
    }

    // If you run this code outside a container, use the env var
    let challenges_path = env::var("CHALLENGES_PATH")
        // default value in container
        .unwrap_or("/app/rustfinity.com/challenges".to_string());


    let repo_path = format!("{challenges_path}/{challenge}");
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
