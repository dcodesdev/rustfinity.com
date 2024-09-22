use base64::prelude::*;
use duct::cmd;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::Instant;

use crate::regex::extract_unittest_path;

pub async fn run_code(code_base64: &str, challenge: &str) -> anyhow::Result<String> {
    let mut output = String::new();

    let tests_output = run_tests(&code_base64, &challenge).await?;
    output.push_str(&tests_output);

    let test_binary_path = extract_unittest_path(&output);

    if let Some(test_binary_path) = test_binary_path {
        let time_output = benchmark_time_min(&challenge, &test_binary_path).await?;
        let memory_output = memory_benchmark(&challenge, &test_binary_path).await?;

        output.push_str("\n");
        output.push_str("---");
        output.push_str("\n");
        output.push_str(time_output.as_str());
        output.push_str("\n");
        output.push_str(memory_output.as_str());
    }

    Ok(output)
}

async fn benchmark_time(challenge: &str, test_binary_path: &str) -> anyhow::Result<f64> {
    let challenges_path = get_challenges_path();
    let current_challenge_path = format!("{challenges_path}/{challenge}");

    let start = Instant::now();

    Command::new(&test_binary_path)
        .current_dir(&current_challenge_path)
        .output()?;

    let elapsed = start.elapsed();
    let as_nanos = elapsed.as_nanos();

    let as_ms = as_nanos as f64 / 1_000_000.0;

    Ok(as_ms)
}

/// Runs the tests 10 times and gets the minimum time
async fn benchmark_time_min(challenge: &str, test_binary_path: &str) -> anyhow::Result<String> {
    let mut nums = Vec::with_capacity(10);

    for _ in 0..10 {
        let time = benchmark_time(&challenge, &test_binary_path).await?;
        nums.push(time);
    }

    // Take Average
    let min = nums
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    let final_output = format!("Time: {:.8}ms", min);

    Ok(final_output)
}

async fn memory_benchmark(challenge: &str, test_binary_path: &str) -> anyhow::Result<String> {
    let challenges_path = get_challenges_path();
    let current_challenge_path = format!("{challenges_path}/{challenge}");

    let output = Command::new("heaptrack")
        .arg(&test_binary_path)
        .current_dir(&current_challenge_path)
        .output()?;

    let stdout = String::from_utf8(output.stdout)?;

    let output_path = stdout.split("\"").collect::<Vec<&str>>()[1];

    // run heaptrack --analyze {output_path}
    let output = Command::new("heaptrack")
        .arg("--analyze")
        .arg(output_path)
        .output()?;

    let stdout = String::from_utf8(output.stdout)?;

    let details = stdout.split("\n\n").collect::<Vec<&str>>();
    let last_index = details.len() - 1;
    let details = details[last_index];

    let mut memory = String::new();

    details.lines().for_each(|line| {
        if line.contains("peak heap memory consumption:") {
            memory = line.replace("peak", "Peak");
        }
    });

    Ok(memory)
}

async fn run_tests(code_base64: &str, challenge: &str) -> anyhow::Result<String> {
    let code_utf8 = BASE64_STANDARD.decode(code_base64)?;
    let code = String::from_utf8(code_utf8)?;

    if challenge == "playground" {
        let cwd = "/app/playground";
        let main_path = Path::new(cwd).join("src/main.rs");

        // Write src/main.rs
        fs::write(&main_path, &code)?;

        // Run the code
        let output = run_command_and_merge_output("cargo", &["run"], Some(cwd)).await?;

        return Ok(output);
    }

    let challenges_path = get_challenges_path();

    let repo_path = format!("{challenges_path}/{challenge}");
    let repository_path = Path::new(&repo_path).canonicalize()?;

    // write src/lib.rs
    let lib_path = repository_path.join("src/lib.rs");

    fs::write(&lib_path, &code)?;

    let cwd = repository_path
        .to_str()
        .ok_or(anyhow::anyhow!("Invalid path"))?;

    let output = run_command_and_merge_output("cargo", &["test"], Some(cwd)).await?;

    Ok(output)
}

fn get_challenges_path() -> String {
    // If you run this code outside a container, use the env var
    env::var("CHALLENGES_PATH")
        // default value in container
        .unwrap_or("/app/rustfinity.com/challenges".to_string())
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
