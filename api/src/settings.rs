use shared_lib::logger::*;


#[derive(Debug)]
pub struct Settings {
    pub log: Log,
    pub actix_config: ActixConfig,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            log: Log::default(),
            actix_config: ActixConfig::default(),
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