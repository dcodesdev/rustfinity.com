use std::{io, process::Command};

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

            let starter_path = format!("challenges/{slug}/src/starter.rs");
            let description_path = format!("challenges/{slug}/description.md");
            let tests_path = format!("challenges/{slug}/tests/tests.rs");

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
    std::fs::File::create(file_path)?;

    Ok(())
}
