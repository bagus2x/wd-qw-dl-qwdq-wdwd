use crate::config::Config;
use crate::internal::model::error::Error;
use crate::internal::model::error::Error::Internal;
use deadpool_redis::{Config as RedisConfig, Connection, Pool, PoolError, Runtime};
use std::sync::Arc;

pub struct Database {
    pub pool: Pool,
}

impl Database {
    pub fn new(config: Arc<Config>) -> Result<Database, Error> {
        let redis_config = RedisConfig::from_url(config.database_url.clone());
        let pool = redis_config
            .create_pool(Some(Runtime::Tokio1))
            .map_err(|err| Internal(err.to_string()))?;

        Ok(Self { pool })
    }

    pub async fn get_connection(&self) -> Result<Connection, PoolError> {
        self.pool.get().await
    }
}
