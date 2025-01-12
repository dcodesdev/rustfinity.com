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

#[test]
fn test_notifications_only() {
    let config = AppConfig {
        notifications_enabled: false,
        ..Default::default()
    };
    assert_eq!(config.theme, "Light");
    assert!(!config.notifications_enabled);
    assert_eq!(config.max_users, 100);
}

#[test]
fn test_max_users_only() {
    let config = AppConfig {
        max_users: 500,
        ..Default::default()
    };
    assert_eq!(config.theme, "Light");
    assert!(config.notifications_enabled);
    assert_eq!(config.max_users, 500);
}

#[test]
fn test_multiple_custom_fields() {
    let config = AppConfig {
        theme: String::from("Dark"),
        max_users: 250,
        ..Default::default()
    };
    assert_eq!(config.theme, "Dark");
    assert!(config.notifications_enabled);
    assert_eq!(config.max_users, 250);
}

#[test]
fn test_all_custom_fields() {
    let config = AppConfig {
        theme: String::from("Blue"),
        notifications_enabled: false,
        max_users: 1000,
        auto_save: false,
        cache_size_mb: 256,
        log_level: String::from("WARN"),
        retry_attempts: 5,
        timeout_seconds: 45,
    };
    assert_eq!(config.theme, "Blue");
    assert!(!config.notifications_enabled);
    assert_eq!(config.max_users, 1000);
    assert!(!config.auto_save);
    assert_eq!(config.cache_size_mb, 256);
    assert_eq!(config.log_level, "WARN");
    assert_eq!(config.retry_attempts, 5);
    assert_eq!(config.timeout_seconds, 45);
}

#[test]
fn test_minimal_changes() {
    let config = AppConfig {
        cache_size_mb: 1024,
        timeout_seconds: 60,
        ..Default::default()
    };
    assert_eq!(config.cache_size_mb, 1024);
    assert_eq!(config.timeout_seconds, 60);
    assert_eq!(config.theme, "Light");
    assert_eq!(config.log_level, "INFO");
    assert_eq!(config.retry_attempts, 3);
    assert!(config.auto_save);
}

#[test]
fn test_production_settings() {
    let config = AppConfig {
        log_level: String::from("ERROR"),
        cache_size_mb: 2048,
        retry_attempts: 5,
        timeout_seconds: 120,
        ..Default::default()
    };
    assert_eq!(config.log_level, "ERROR");
    assert_eq!(config.cache_size_mb, 2048);
    assert_eq!(config.retry_attempts, 5);
    assert_eq!(config.timeout_seconds, 120);
    assert!(config.notifications_enabled);
}

#[test]
fn test_debug_settings() {
    let config = AppConfig {
        log_level: String::from("DEBUG"),
        cache_size_mb: 256,
        timeout_seconds: 10,
        auto_save: false,
        ..Default::default()
    };
    assert_eq!(config.log_level, "DEBUG");
    assert_eq!(config.cache_size_mb, 256);
    assert_eq!(config.timeout_seconds, 10);
    assert!(!config.auto_save);
    assert_eq!(config.retry_attempts, 3);
}
