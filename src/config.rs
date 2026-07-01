use anyhow::{anyhow, Result};
use std::env;
use std::time::Duration;

#[derive(Clone)]
pub struct Config {
    pub kafka_brokers: String,
    pub kafka_topic: String,
    pub batch_size: usize,
    pub batch_timeout: Duration,
    pub db_url: String,
    pub db_name: String,
    pub db_user: String,
    pub db_pass: String,
}

pub fn from_environment() -> Result<Config> {
    dotenvy::dotenv().ok();

    let kafka_brokers = env::var("KAFKA_BROKERS")
        .map_err(|_| anyhow!("KAFKA_BROKERS environment variable not set"))?;
    
    let kafka_topic = env::var("KAFKA_TOPIC")
        .map_err(|_| anyhow!("KAFKA_TOPIC environment variable not set"))?;

    let batch_size = env::var("BATCH_SIZE")
        .map_err(|_| anyhow!("BATCH_SIZE environment variable not set"))?
        .parse::<usize>()
        .ok()
        .unwrap_or(1);

    let batch_timeout = Duration::from_millis(
        env::var("BATCH_TIMEOUT")
            .map_err(|_| anyhow!("BATCH_TIMEOUT environment variable not set"))?
            .parse::<u64>()
            .ok()
            .unwrap_or(5000)
    );

    let db_url = env::var("DB_URL")
        .map_err(|_| anyhow!("DB_URL environment variable not set"))?;

    let db_name = env::var("DB_NAME")
        .map_err(|_| anyhow!("DB_NAME environment variable not set"))?;

    let db_user = env::var("DB_USER")
        .map_err(|_| anyhow!("DB_USER environment variable not set"))?;

    let db_pass = env::var("DB_PASS")
        .map_err(|_| anyhow!("DB_PASS environment variable not set"))?;

    Ok(Config {
        kafka_brokers,
        kafka_topic,
        batch_size,
        batch_timeout,
        db_url,
        db_name,
        db_user,
        db_pass,
    })
}