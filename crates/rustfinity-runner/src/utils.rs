use base64::{prelude::BASE64_STANDARD, Engine};
use duct::cmd;

pub fn to_utf8(base64: &str) -> anyhow::Result<String> {
    let utf8 = BASE64_STANDARD.decode(base64)?;
    Ok(String::from_utf8(utf8)?)
}

pub async fn run_command_and_merge_output(
    command: &str,
    args: &[&str],
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
