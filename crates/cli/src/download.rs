use crate::challenge::challenge_exists;
use dload::Downloader;
use futures::future::join_all;
use std::error::Error;
use std::fs;

const FILES: [&'static str; 4] = [
    "description.md",
    "Cargo.toml",
    "src/starter.rs",
    "src/tests.rs",
];

const GITHUB_BASE_URL: &'static str =
    "https://raw.githubusercontent.com/dcodesdev/rustfinity.com/main/challenges";

pub async fn get_challenge(challenge: &str) -> anyhow::Result<()> {
    if !challenge_exists(challenge).await {
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

async fn download_file(url: &str, challenge: &str) -> Result<Downloader, Box<dyn Error>> {
    let is_src = url.contains("src");

    let output_dir = if is_src {
        format!("{}/src", challenge)
    } else {
        challenge.to_string()
    };

    let file_name = url.split("/").last().unwrap();

    let dl = Downloader::new();

    if file_name == "starter.rs" {
        let dl = dl
            .set_output_dir(&output_dir)
            .file_name("lib.rs")
            .download(url)
            .await;

        // After the download, prepend "pub mod tests;\n" to the file
        let file_path = format!("{}/lib.rs", output_dir);
        let contents = fs::read_to_string(&file_path).unwrap();
        let new_contents = format!("pub mod tests;\n\n{}", contents);
        fs::write(&file_path, new_contents).unwrap();

        dl
    } else {
        dl.set_output_dir(&output_dir).download(url).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            fs::create_dir_all("temp/test_downloads_challenge").ok();
            env::set_current_dir("temp/test_downloads_challenge").ok();

            for challenge in CHALLENGES {
                get_challenge(challenge)
                    .await
                    .expect("Failed to download challenge");

                let paths_to_exist = ["description.md", "Cargo.toml", "src/lib.rs", "src/tests.rs"];

                for file in paths_to_exist.iter() {
                    let path = format!("{}/{}", challenge, file);

                    assert!(Path::new(&path).exists());

                    // all files shouldn't have the content "404: Not Found"
                    let contents = fs::read_to_string(&path).unwrap();
                    assert!(!contents.contains("404: Not Found"));
                }
            }
        }
    }

    mod download_file {
        use std::{env, path::Path};

        use super::*;

        #[tokio::test]
        async fn test_downloads_file() {
            fs::create_dir_all("temp/test_downloads_file").ok();
            env::set_current_dir("temp/test_downloads_file").ok();

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
            fs::create_dir_all("temp/test_download_file_sub_dir").ok();
            env::set_current_dir("temp/test_download_file_sub_dir").ok();

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
