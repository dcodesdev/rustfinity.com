use crate::constants::GITHUB_REPO_URL;

pub fn update_dependency_if_exists(cargo_toml: &mut String) -> anyhow::Result<()> {
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

    *cargo_toml = updated_content;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_dependency_if_exists() {
        let mut cargo_toml = r#"
[package]
name = "rustfinity"
version = "0.1.0"

[dependencies]
syntest = "1.0"
"#
        .to_string();

        // update existing dependency
        update_dependency_if_exists(&mut cargo_toml).unwrap();

        let expected = r#"
[package]
name = "rustfinity"
version = "0.1.0"

[dependencies]
syntest = { git = "https://www.github.com/dcodesdev/rustfinity.com" }
"#;

        assert_eq!(cargo_toml, expected);
    }
}
