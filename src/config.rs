use crate::utility::rts;

pub fn load_config() -> Config {
    #[cfg(debug_assertions)]
    let config_str = rts!("config/config.dev.toml");
    #[cfg(not(debug_assertions))]
    let config_str = rts!("config/config.toml");

    toml::from_str(&config_str).unwrap()
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct Config {
    pub database: Database,
    pub backend: Backend,
    pub user_agent: std::collections::HashMap<String, String>,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct Database {
    pub port: u16,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct Backend {
    pub port: u16,
}
