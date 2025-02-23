use crate::config::Config;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{Error, MySql, Pool};
use std::sync::Arc;

pub struct MysqlDatabase {
    pub pool: Pool<MySql>,
}

impl MysqlDatabase {
    pub async fn new(config: Arc<Config>) -> Result<MysqlDatabase, Error> {
        let pool = MySqlPoolOptions::new()
            .max_connections(config.max_connections)
            .min_connections(config.min_connections)
            .acquire_timeout(config.acquire_timeout)
            .idle_timeout(config.idle_timeout)
            .connect(&config.database_url)
            .await?;

        sqlx::query("SELECT 1+1").execute(&pool).await?;

        Ok(Self { pool })
    }
}
