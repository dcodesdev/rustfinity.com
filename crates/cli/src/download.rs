use crate::challenge::challenge_exists;
use dload::Downloader;
use futures::future::join_all;

const FILES: [&'static str; 4] = [
    "description.md",
    "Cargo.toml",
    "src/starter.rs",
    "tests/tests.rs",
];

const GITHUB_BASE_URL: &'static str =
    "https://raw.githubusercontent.com/dcodesdev/rustfinity.com/main/challenges";

pub async fn get_challenge(challenge: &str) -> anyhow::Result<()> {
    if !challenge_exists(challenge).await? {
        println!("Challenge does not exist 🥺\n\nPlease make sure you've written the challenge name correctly.");
        return Ok(());
    }

    let futures: Vec<_> = FILES
        .iter()
        .map(|file| {
            let url = format!("{}/{}/{}", GITHUB_BASE_URL, challenge, file);
            let challenge = challenge.to_string();
            async move { download_file(&url, &challenge).await }
        })
        .collect();

    let results = join_all(futures).await;

    // Check all results are successful
    if results.iter().all(Result::is_ok) {
        println!(
            "Challenge downloaded 🥳\n\nRun the following command to get started:\n\ncd {}",
            challenge
        );
        Ok(())
    } else {
        Err(anyhow::anyhow!("One or more files failed to download"))
    }
}

async fn download_file(url: &str, challenge: &str) -> anyhow::Result<Downloader> {
    let is_src = url.contains("/src/");
    let is_test = url.contains("/tests/");

    let output_dir = if is_src {
        format!("{}/src", challenge)
    } else if is_test {
        format!("{}/tests", challenge)
    } else {
        challenge.to_string()
    };

    let file_name = url
        .split("/")
        .last()
        .ok_or(anyhow::anyhow!("Failed to get file name"))?;

    let dl = Downloader::new();

    let file_name = if file_name == "starter.rs" {
        "lib.rs"
    } else {
        file_name
    };

    dl.set_output_dir(&output_dir)
        .file_name(&file_name)
        .download(url)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to download file: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    mod download {
        const CHALLENGES: [&'static str; 7] = [
            "hello-world",
            "character-counting-string",
            "mathematical-operations",
            "fizz-buzz",
            "fibonacci",
            "the-from-trait",
            "animal-sanctuary-registry",
        ];

        use super::*;
        use std::{env, fs, path::Path};

        #[tokio::test]
        async fn test_downloads_challenge() {
            // rm temp dir if exists
            let temp_dir = tempdir().expect("Failed to create temp dir");
            let temp_path = temp_dir.path();
            env::set_current_dir(&temp_path).ok();

            let test_challenge = |challenge: String| async move {
                get_challenge(&challenge)
                    .await
                    .expect("Failed to download challenge");

                let paths_to_exist = [
                    "description.md",
                    "Cargo.toml",
                    "src/lib.rs",
                    "tests/tests.rs",
                ];

                for file in paths_to_exist.iter() {
                    let path = Path::new(&challenge).join(file);
                    assert!(path.exists(), "File does not exist: {:?}", path);

                    // all files shouldn't have the content "404: Not Found"
                    let contents = fs::read_to_string(&path).unwrap();
                    assert!(!contents.contains("404: Not Found"));
                }
            };

            let handles = CHALLENGES
                .iter()
                .map(|c| tokio::spawn(test_challenge(c.to_string())))
                .collect::<Vec<_>>();

            futures::future::join_all(handles).await;
        }
    }

    mod download_file {
        use super::*;
        use std::{env, fs, path::Path};

        #[tokio::test]
        async fn test_downloads_file() {
            let temp_dir = tempdir().expect("Failed to create temp dir");
            let temp_path = temp_dir.path();
            env::set_current_dir(&temp_path).ok();

            let url = "https://raw.githubusercontent.com/dcodesdev/rustfinity.com/main/challenges/hello-world/description.md";
            let challenge = "hello-world";

            let result = download_file(url, challenge).await;

            assert!(result.is_ok());

            let path = format!("{}/description.md", challenge);
            assert!(Path::new(&path).exists());

            // read the contents of the file
            let contents = fs::read_to_string(&path).unwrap();

            // the file shouldn't have the content "404: Not Found"
            assert!(!contents.contains("404: Not Found"));
        }

        #[tokio::test]
        async fn test_download_file_sub_dir() {
            let temp_dir = tempdir().expect("Failed to create temp dir");
            let temp_path = temp_dir.path();
            env::set_current_dir(&temp_path).ok();

            let url = "https://raw.githubusercontent.com/dcodesdev/rustfinity.com/main/challenges/hello-world/src/starter.rs";
            let challenge = "hello-world";

            let result = download_file(url, challenge).await;

            assert!(result.is_ok());

            let path = format!("{}/src/lib.rs", challenge);
            assert!(Path::new(&path).exists());

            // read the contents of the file
            let contents = fs::read_to_string(&path).unwrap();

            // the file shouldn't have the content "404: Not Found"
            assert!(!contents.contains("404: Not Found"));
        }
    }
}
