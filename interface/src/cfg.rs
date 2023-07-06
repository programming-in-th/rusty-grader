use config::Environment;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub database_config: DatabaseConfig,
    pub rmq_config: RabbitMqConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: i32,
    pub username: String,
    pub password: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RabbitMqConfig {
    pub host: String,
    pub port: i32,
    pub username: String,
    pub password: String,
    pub vhost: String,
    pub env: String,
}

pub fn read_config() -> Result<AppConfig, Box<dyn std::error::Error>> {
    let database_config = config::Config::builder()
        .add_source(Environment::with_prefix("DATABASE").prefix_separator("_"))
        .build()?
        .try_deserialize()?;

    let rmq_config = config::Config::builder()
        .add_source(Environment::with_prefix("RABBITMQ").prefix_separator("_"))
        .build()?
        .try_deserialize()?;

    Ok(AppConfig {
        database_config,
        rmq_config,
    })
}
