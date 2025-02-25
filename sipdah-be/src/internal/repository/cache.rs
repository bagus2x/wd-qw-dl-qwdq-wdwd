use crate::config::Config;
use crate::internal::model;
use crate::internal::model::error::Error;
use chrono::Duration;
use deadpool_redis::{redis::cmd, Pool};
use model::cache;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;
use std::sync::Arc;

pub struct Repository {
    config: Arc<Config>,
    pool: Arc<Pool>,
}

impl Repository {
    pub fn new(config: Arc<Config>, pool: Arc<Pool>) -> Self {
        Self { config, pool }
    }
}

impl cache::Repository for Repository {
    async fn set<T: Serialize>(&self, key: String, value: T) -> Result<(), Error> {
        self.setx(key, value, self.config.redis_default_ttl).await
    }

    async fn setx<T: Serialize>(&self, key: String, value: T, ttl: Duration) -> Result<(), Error> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|err| Error::Internal(err.to_string()))?;
        let serialized =
            serde_json::to_string(&value).map_err(|err| Error::Internal(err.to_string()))?;

        cmd("SETEX")
            .arg(key)
            .arg(ttl.num_seconds())
            .arg(serialized)
            .query_async::<()>(&mut conn)
            .await
            .map_err(|err| Error::Internal(err.to_string()))?;

        Ok(())
    }

    async fn get<T: DeserializeOwned>(&self, key: String) -> Result<Option<T>, Error> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|err| Error::Internal(err.to_string()))?;
        let value: Option<String> = cmd("GET").arg(key).query_async(&mut conn).await.ok();

        match value {
            Some(serialized) => {
                let value = serde_json::from_str::<T>(serialized.as_str()).unwrap();
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }

    async fn del(&self, key: String) -> Result<(), Error> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|err| Error::Internal(err.to_string()))?;

        cmd("DEL")
            .arg(key)
            .query_async::<()>(&mut conn)
            .await
            .map_err(|err| Error::Internal(err.to_string()))?;

        Ok(())
    }
}
