use config::ConfigError;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Postgres {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
    pub ssl_mode: String,
    pub migrate: bool,
    pub debug: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub postgres: Postgres,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let settings = Config::builder()
            .add_source(config::File::with_name("config.yml"))
            .build()
            .unwrap();

        let cfg = settings.try_deserialize::<Self>()?;

        Ok(cfg)
    }
}