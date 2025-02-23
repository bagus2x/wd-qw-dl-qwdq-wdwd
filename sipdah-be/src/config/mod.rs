use std::env;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub database_url: String,
    pub idle_timeout: Duration,
    pub acquire_timeout: Duration,
    pub max_connections: u32,
    pub min_connections: u32,
    pub access_token_key_secret: String,
    pub refresh_token_key_secret: String,
}

impl Config {
    pub fn new() -> Self {
        Config {
            port: env::var("PORT")
                .map(|v| v.parse::<u16>().unwrap_or(8080))
                .expect("PORT must be set"),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            idle_timeout: env::var("IDLE_TIMEOUT")
                .map(|v| v.parse::<u64>().unwrap_or(30))
                .map(Duration::from_secs)
                .expect("IDLE_TIMEOUT must be set and valid"),
            acquire_timeout: env::var("ACQUIRE_TIMEOUT")
                .map(|v| v.parse::<u64>().unwrap_or(15))
                .map(Duration::from_secs)
                .expect("ACQUIRE_TIMEOUT must be set and valid"),
            max_connections: env::var("MAX_CONNECTIONS")
                .map(|v| v.parse::<u32>().unwrap_or(10))
                .expect("MAX_CONNECTIONS must be set"),
            min_connections: env::var("MIN_CONNECTIONS")
                .map(|v| v.parse::<u32>().unwrap_or(1))
                .expect("MIN_CONNECTIONS must be set"),
            access_token_key_secret: env::var("ACCESS_TOKEN_KEY")
                .expect("ACCESS_TOKEN_KEY must be set"),
            refresh_token_key_secret: env::var("REFRESH_TOKEN_KEY")
                .expect("REFRESH_TOKEN_KEY must be set"),
        }
    }
}
