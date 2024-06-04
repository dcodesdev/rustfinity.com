pub async fn get_latest_version() -> anyhow::Result<String> {
    let url = "https://crates.io/api/v1/crates/rustfinity";

    let client = reqwest::ClientBuilder::new()
        .user_agent("rustfinity")
        .build()?;

    let response = client.get(url).send().await?;
    let json: serde_json::Value = response.json().await?;

    let latest_version = json["crate"]["newest_version"]
        .as_str()
        .ok_or(anyhow::anyhow!("Failed to get latest version"))?;

    Ok(latest_version.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_latest_version() {
        let version = get_latest_version().await.unwrap();
        assert!(!version.is_empty());

        // Expect the pattern to be "x.x.x" while x is a number
        let mut version = version.split('.');
        assert!(version.next().unwrap().parse::<u32>().is_ok());
        assert!(version.next().unwrap().parse::<u32>().is_ok());
        assert!(version.next().unwrap().parse::<u32>().is_ok());
        assert!(version.next().is_none());
    }
}
