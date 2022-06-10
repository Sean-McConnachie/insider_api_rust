#[derive(Debug)]
pub struct Settings {
    pub actix_config: ActixConfig,
    pub database_url: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            actix_config: ActixConfig::default(),
            database_url: "postgres://postgres:12341234@127.0.0.1:5432/insider_api_rust".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct ActixConfig {
    pub url: String,
    pub port: u16,
}

impl Default for ActixConfig {
    fn default() -> Self {
        ActixConfig {
            url: "127.0.0.1".to_string(),
            port: 8080,
        }
    }
}