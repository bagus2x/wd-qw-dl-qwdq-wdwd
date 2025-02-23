use crate::internal::model::error::Error;
use chrono::DateTime;
use chrono::Local;
use serde::Serialize;
use sqlx::FromRow;

#[derive(FromRow, Serialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: String,
    pub name: String,
    pub phone_number: Option<String>,
    pub photo_url: Option<String>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}

pub trait Repository {
    async fn create(&self, user: &User) -> Result<(), Error>;

    async fn find_by_id(&self, user_id: &str) -> Result<User, Error>;

    async fn exists_by_email(&self, email: &str) -> Result<bool, Error>;
}
