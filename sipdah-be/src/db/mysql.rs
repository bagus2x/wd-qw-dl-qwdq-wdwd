use crate::config::Config;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{Error, MySql, Pool};
use std::sync::Arc;

pub struct Database {
    pub pool: Pool<MySql>,
}

impl Database {
    pub async fn new(config: Arc<Config>) -> Result<Database, Error> {
        let pool = MySqlPoolOptions::new()
            .max_connections(config.max_connections)
            .min_connections(config.min_connections)
            .acquire_timeout(config.acquire_timeout.to_std().unwrap())
            .idle_timeout(config.idle_timeout.to_std().unwrap())
            .connect(&config.database_url)
            .await?;

        sqlx::query("SELECT 1+1").execute(&pool).await?;

        Ok(Self { pool })
    }
}
