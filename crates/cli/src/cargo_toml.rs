use crate::constants::GITHUB_REPO_URL;

pub fn update_dependency_if_exists(cargo_toml: &str) -> anyhow::Result<String> {
    let mut updated_content = cargo_toml
        .lines()
        .map(|line| {
            if line.contains("syntest") {
                format!("syntest = {{ git = \"{}\" }}", GITHUB_REPO_URL)
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join("\n");

    updated_content.push_str("\n");

    Ok(updated_content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_dependency_if_exists() {
        let cargo_toml = r#"
[package]
name = "rustfinity"
version = "0.1.0"

[dependencies]
syntest = "1.0"
"#;

        // update existing dependency
        let actual = update_dependency_if_exists(&cargo_toml).unwrap();

        let expected = r#"
[package]
name = "rustfinity"
version = "0.1.0"

[dependencies]
syntest = { git = "https://www.github.com/dcodesdev/rustfinity.com" }
"#;

        assert_eq!(actual, expected);
    }
}
