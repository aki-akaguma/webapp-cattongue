use serde::Deserialize;
use std::sync::OnceLock;

#[derive(Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub database: DatabaseConfig,
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
    pub backend_url: String,
}

pub static CONFIG: OnceLock<AppConfig> = OnceLock::new();

impl AppConfig {
    pub fn load() -> anyhow::Result<Self> {
        let config_str = std::fs::read_to_string("config.toml").unwrap_or_else(|_| {
            // デフォルト設定
            r#"
[database]
base_path = "/var/local/data/cattongue"
db_file = "cattongue.sqlite3"
session_db_file = "sessions.sqlite3"

[client]
backend_url = "https://aki.omusubi.org/cattongue"
"#
            .to_string()
        });
        let mut config: AppConfig = toml::from_str(&config_str)?;

        // Overriding with environment variables
        if let Ok(val) = std::env::var("CATTONGUE_DB_BASE_PATH") {
            config.database.base_path = val;
        }
        if let Ok(val) = std::env::var("CATTONGUE_DB_FILE") {
            config.database.db_file = val;
        }
        if let Ok(val) = std::env::var("CATTONGUE_DB_SESSION_PATH") {
            config.database.session_db_file = val;
        }
        if let Ok(val) = std::env::var("CATTONGUE_BACKEND_URL") {
            config.client.backend_url = val;
        }

        Ok(config)
    }

    #[cfg(feature = "server")]
    pub fn global() -> &'static AppConfig {
        CONFIG.get().expect("Config is not initialized")
    }
}
