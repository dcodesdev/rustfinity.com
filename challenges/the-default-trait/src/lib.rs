#[derive(Debug)]
pub struct AppConfig {
    pub theme: String,
    pub notifications_enabled: bool,
    pub max_users: u32,
    pub auto_save: bool,
    pub cache_size_mb: u32,
    pub log_level: String,
    pub retry_attempts: u32,
    pub timeout_seconds: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            theme: String::from("Light"),
            notifications_enabled: true,
            max_users: 100,
            auto_save: true,
            cache_size_mb: 512,
            log_level: String::from("INFO"),
            retry_attempts: 3,
            timeout_seconds: 30,
        }
    }
}
