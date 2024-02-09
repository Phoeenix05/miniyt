pub fn load_config() -> Config {
    let config_str = std::fs::read_to_string("config/config.toml").unwrap();
    let config: Config = toml::from_str(&config_str).unwrap();
    config
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
