use serde::Deserialize;

#[derive(Deserialize)]
enum Difficulty {
    BEGINNER,
    EASY,
    MEDIUM,
    HARD,
    ADVANCED,
}

#[derive(Deserialize)]
#[allow(non_camel_case_types)]
enum Track {
    RUST_BASICS,
    CONTROL_FLOW,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;
    use std::{fs, path::PathBuf};

    #[derive(Deserialize)]
    struct Package {
        pub name: String,
    }

    #[derive(Deserialize)]
    struct CargoToml {
        pub package: Package,
    }

    #[derive(Deserialize)]
    #[allow(unused)]
    struct Challenge {
        pub id: u32,
        pub title: String,
        pub slug: String,
        pub short_description: String,
        pub language: String,
        pub difficulty: Difficulty,
        pub track: Track,
        pub tags: Vec<String>,
        pub created_at: String,
        pub updated_at: String,
    }

    fn challenges_json() -> Result<Vec<Challenge>, std::io::Error> {
        let challenges_str = fs::read_to_string("challenges.json")?;
        let challenges = serde_json::from_str::<Vec<Challenge>>(&challenges_str)?;
        Ok(challenges)
    }

    fn challenges_dir_list() -> Result<Vec<PathBuf>, std::io::Error> {
        let mut entries = fs::read_dir("../challenges")?;
        let mut dirs = vec![];
        let ignored_dirs = ["src/", "target/"];

        while let Some(entry) = entries.next() {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let abs_path = path.canonicalize().unwrap();

                if ignored_dirs.iter().any(|&dir| abs_path.ends_with(dir)) {
                    continue;
                }

                dirs.push(abs_path);
            }
        }

        Ok(dirs)
    }

    #[test]
    fn test_read_challenges() {
        let challenges = challenges_json().expect("Failed to read challenges");
        assert!(!challenges.is_empty(), "Expected some challenges");
    }

    #[test]
    fn test_dirs() {
        let challenges = challenges_dir_list().expect("Failed to read challenges directories");
        for challenge_dir in challenges {
            let dir_name = challenge_dir.file_name().unwrap().to_str().unwrap();

            let description_md = challenge_dir.join("description.md");
            let cargo_toml = challenge_dir.join("Cargo.toml");
            let src_lib = challenge_dir.join("src/lib.rs");
            let src_starter = challenge_dir.join("src/starter.rs");

            // let src_tests = challenge_dir.join("src/tests.rs");
            let tests_exist = challenge_dir.join("src/tests.rs").exists()
                || challenge_dir.join("tests/tests.rs").exists();

            assert!(
                description_md.exists(),
                "Missing description.md in {}",
                dir_name
            );
            assert!(cargo_toml.exists(), "Missing Cargo.toml in {}", dir_name);
            assert!(src_lib.exists(), "Missing src/lib.rs in {}", dir_name);
            assert!(tests_exist, "Missing tests, you should have either src/tests.rs or tests/tests.rs available in {}", dir_name);
            assert!(
                src_starter.exists(),
                "Missing src/starter.rs in {}",
                dir_name
            );

            let cargo_content = fs::read_to_string(&cargo_toml).unwrap();
            let cargo: CargoToml = toml::from_str(&cargo_content).unwrap();
            let package_name = cargo.package.name;

            assert_eq!(
                package_name, dir_name,
                "Cargo.toml package name does not match directory name {}",
                dir_name
            );

            let challenges = challenges_json().expect("Failed to read challenges");
            let challenge = challenges.iter().find(|c| c.slug == dir_name);

            assert!(
                challenge.is_some(),
                "Missing entry in challenges.json for {}",
                dir_name
            );
        }
    }

    #[test]
    fn test_duplicate_ids() {
        let challenges = challenges_json().expect("Failed to read challenges");
        let mut ids = vec![];

        for challenge in challenges {
            if ids.contains(&challenge.id) {
                panic!("Duplicate id found: {}", challenge.slug);
            }

            ids.push(challenge.id);
        }
    }

    #[test]
    fn test_duplicate_slugs() {
        let challenges = challenges_json().expect("Failed to read challenges");
        let mut slugs = vec![];

        for challenge in challenges {
            if slugs.contains(&challenge.slug) {
                panic!("Duplicate slug found: {}", challenge.slug);
            }

            slugs.push(challenge.slug);
        }
    }

    #[test]
    fn test_date_validity() {
        let challenges: Vec<Challenge> = challenges_json().expect("Failed to read challenges");

        for challenge in challenges {
            let created_at: NaiveDateTime =
                NaiveDateTime::parse_from_str(&challenge.created_at, "%Y-%m-%dT%H:%M:%S%Z")
                    .expect("Failed to parse created_at date");

            let updated_at: NaiveDateTime =
                NaiveDateTime::parse_from_str(&challenge.updated_at, "%Y-%m-%dT%H:%M:%S%Z")
                    .expect("Failed to parse updated_at date");

            assert!(
                created_at <= updated_at,
                "created_at date should be before updated_at date"
            );
        }
    }
}
