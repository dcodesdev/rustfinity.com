use std::{io, path::Path, process::Command};

use challenges::{challenges_json, get_max_id};

fn main() {
    let mut args = std::env::args();
    args.next(); // Skip the first argument

    let arg = args.next().unwrap_or("".to_string());

    match arg.as_str() {
        "count" => {
            let challenges = challenges_json().expect("Failed to read challenges.json");

            println!("Total challenges: {}", challenges.len());
        }
        "create" | "c" => {
            let slug = args.next().expect("Slug is required");

            Command::new("cargo")
                .arg("new")
                .arg("--lib")
                .arg(slug.as_str())
                .output()
                .expect("Failed to create new project");

            let starter_path = format!("{slug}/src/starter.rs");
            let description_path = format!("{slug}/description.md");
            let tests_path = format!("{slug}/tests/tests.rs");

            create_file(starter_path.as_str()).expect("Failed to create starter.rs");
            create_file(description_path.as_str()).expect("Failed to create description.md");
            create_file(tests_path.as_str()).expect("Failed to create tests.rs");
        }
        _ => {
            let max_id = get_max_id();

            println!("Max id is: {}", max_id);
        }
    }
}

fn create_file(file_path: &str) -> Result<(), io::Error> {
    let parent = Path::new(file_path)
        .parent()
        .expect("Failed to get parent directory");

    std::fs::create_dir_all(parent)?;
    std::fs::File::create(file_path)?;

    Ok(())
}
