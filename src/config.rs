use config::{Config, Environment, File, FileFormat};
use serde::Deserialize;
use std::sync::OnceLock;

#[derive(Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    #[allow(dead_code)]
    pub client: ClientConfig,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DatabaseConfig {
    pub base_path: String,
    pub db_file: String,
    pub session_db_file: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ClientConfig {
    #[allow(dead_code)]
    pub backend_url: String,
}

pub static CONFIG: OnceLock<AppConfig> = OnceLock::new();

impl AppConfig {
    pub fn load() -> anyhow::Result<Self> {
        let default_toml = r#"
[database]
base_path = "/var/local/data/cattongue"
db_file = "cattongue.sqlite3"
session_db_file = "sessions.sqlite3"

[client]
backend_url = "https://aki.omusubi.org/cattongue"
"#;

        let s = Config::builder()
            // 1. Load defaults
            .add_source(File::from_str(default_toml, FileFormat::Toml))
            // 2. Load from config.toml if it exists
            .add_source(File::with_name("config").required(false))
            // 3. Environment variables (e.g., CATTONGUE__DATABASE__BASE_PATH)
            .add_source(Environment::with_prefix("CATTONGUE").separator("__"))
            .build()?;

        let config: AppConfig = s.try_deserialize()?;
        Ok(config)
    }

    pub fn global() -> &'static AppConfig {
        CONFIG.get().expect("Config is not initialized")
    }
}
