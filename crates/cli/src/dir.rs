use std::env;

pub fn get_current_dir() -> anyhow::Result<String> {
    let current_dir = env::current_dir().unwrap();

    let current_dir = current_dir
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| anyhow::anyhow!("Failed to get directory name"))?;

    Ok(current_dir.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_current_dir() {
        let current_dir = get_current_dir();

        assert_eq!(current_dir.unwrap(), "cli")
    }
}
