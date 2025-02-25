use crate::internal::model::error::Error;
use chrono::Duration;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait Repository {
    async fn set<T: Serialize>(&self, key: String, value: T) -> Result<(), Error>;

    async fn setx<T: Serialize>(&self, key: String, value: T, ttl: Duration) -> Result<(), Error>;

    async fn get<T: DeserializeOwned>(&self, key: String) -> Result<Option<T>, Error>;

    async fn del(&self, key: String) -> Result<(), Error>;
}
