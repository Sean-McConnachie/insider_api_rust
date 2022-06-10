#[derive(Debug)]
pub struct Settings {
    pub database_url: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            database_url: "postgres://postgres:12341234@127.0.0.1:5432/insider_api_rust".to_string()
        }
    }
}