use the_default_trait::*;

#[test]
fn test_default_config() {
    let config = AppConfig::default();
    assert_eq!(config.theme, "Light");
    assert!(config.notifications_enabled);
    assert_eq!(config.max_users, 100);
}

#[test]
fn test_custom_config() {
    let config = AppConfig {
        theme: String::from("Dark"),
        ..Default::default()
    };
    assert_eq!(config.theme, "Dark");
    assert!(config.notifications_enabled);
    assert_eq!(config.max_users, 100);
}
