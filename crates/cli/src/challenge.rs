pub async fn challenge_exists(challenge: &str) -> bool {
    let url = "https://raw.githubusercontent.com/dcodesdev/rustfinity.com/main";
    let url = format!("{}/challenges/{}/description.md", url, challenge);

    let client = reqwest::Client::new();

    let text = client.get(url).send().await.unwrap().text().await.unwrap();

    text != "404: Not Found"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_challenge_exists() {
        let slug = "two-sum";
        assert_eq!(challenge_exists(slug).await, false);

        let slug = "hello-world";
        assert_eq!(challenge_exists(slug).await, true);
    }
}
