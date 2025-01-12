#[derive(Debug)]
pub struct AppConfig {
    pub theme: String,
    pub notifications_enabled: bool,
    pub max_users: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            theme: String::from("Light"),
            notifications_enabled: true,
            max_users: 100,
        }
    }
}
