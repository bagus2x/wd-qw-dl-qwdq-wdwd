use crate::internal::model::error::Error;
use chrono::{DateTime, Local};
use sqlx::FromRow;

pub const ROLE_USER: &str = "USER";

#[derive(FromRow)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}

pub trait Repository {
    async fn create(&self, user: &Role) -> Result<(), Error>;

    async fn find_by_id(&self, user_id: &str) -> Result<Role, Error>;

    async fn find_by_name(&self, name: &str) -> Result<Role, Error>;

    async fn exists_by_name(&self, name: &str) -> Result<bool, Error>;

    async fn add(&self, user_id: &str, role_id: &str) -> Result<(), Error>;
}
