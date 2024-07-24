use std::fs;
use toml::from_str;

pub fn read_cargo_toml() -> anyhow::Result<toml::Value> {
    let cargo_toml = fs::read_to_string("Cargo.toml")?;
    let cargo_toml: toml::Value = from_str(&cargo_toml)?;

    Ok(cargo_toml)
}

/// Returns a list of dependencies with their values.
/// Example: [("anyhow", "1.0"), ("clap", "2.33")]
pub fn read_dependencies(cargo_toml: &toml::Value) -> anyhow::Result<Vec<(String, String)>> {
    let dependencies = cargo_toml
        .get("dependencies")
        .ok_or(anyhow::anyhow!("Failed to get dependencies"))?;

    let dependencies = dependencies
        .as_table()
        .ok_or(anyhow::anyhow!("Failed to get dependencies"))?;

    let dependencies = dependencies
        .iter()
        .map(|(name, version)| (name.to_string(), version.as_str().unwrap().to_string()))
        .collect();

    Ok(dependencies)
}

pub fn update_dependency(
    cargo_toml: &mut toml::Value,
    (dependency, version): (&str, &str),
) -> anyhow::Result<()> {
    let dependencies = cargo_toml
        .get_mut("dependencies")
        .ok_or(anyhow::anyhow!("Failed to get dependencies"))?;

    dependencies.as_table_mut().unwrap().insert(
        dependency.to_string(),
        toml::Value::String(version.to_string()),
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_cargo_toml() {
        let cargo_toml = r#"
            [package]
            name = "rustfinity"
            version = "0.1.0"
            authors = ["Your Name <yourname@gmail.com>"]
            edition = "2018"

            [dependencies]
            anyhow = "1.0"
            clap = "2.33"
        "#;

        let expected_dependencies = vec![
            ("anyhow".to_string(), "1.0".to_string()),
            ("clap".to_string(), "2.33".to_string()),
        ];

        let cargo_toml = toml::from_str(cargo_toml).unwrap();
        let dependencies = read_dependencies(&cargo_toml).unwrap();

        assert_eq!(dependencies, expected_dependencies);
    }

    #[test]
    fn test_update_dependency() {
        let cargo_toml = r#"
            [package]
            name = "rustfinity"
            version = "0.1.0"

            [dependencies]
            anyhow = "1.0"
            clap = "2.33"
        "#;

        let mut cargo_toml = toml::from_str(cargo_toml).unwrap();

        // insert new dependency
        update_dependency(&mut cargo_toml, ("tokio", "1.0")).unwrap();

        let dependencies = read_dependencies(&cargo_toml).unwrap();

        assert_eq!(
            dependencies,
            vec![
                ("anyhow".to_string(), "1.0".to_string()),
                ("clap".to_string(), "2.33".to_string()),
                ("tokio".to_string(), "1.0".to_string()),
            ]
        );

        // update existing dependency
        update_dependency(&mut cargo_toml, ("clap", "2.5")).unwrap();

        let dependencies = read_dependencies(&cargo_toml).unwrap();

        assert_eq!(
            dependencies,
            vec![
                ("anyhow".to_string(), "1.0".to_string()),
                ("clap".to_string(), "2.5".to_string()),
                ("tokio".to_string(), "1.0".to_string()),
            ]
        );
    }
}
