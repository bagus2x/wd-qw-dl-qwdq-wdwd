use crate::internal::model::error::Error;
use chrono::DateTime;
use chrono::Local;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

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

    async fn find_by_id(&self, user_id: &str) -> Result<Option<User>, Error>;

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error>;

    async fn exists_by_email(&self, email: &str) -> Result<bool, Error>;

    async fn exists_by_id(&self, id: &str) -> Result<bool, Error>;
}

pub trait Service {
    async fn get_by_id(&self, user_id: &str) -> Result<UserResponse, Error>;

    async fn get_current(&self) -> Result<UserResponse, Error>;

    async fn add_roles(&self, req: AddRolesRequest) -> Result<UserResponse, Error>;
}

#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub name: String,
    pub phone_number: Option<String>,
    pub photo_url: Option<String>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

#[derive(Validate, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct AddRolesRequest {
    pub user_id: String,
    pub roles: Vec<String>,
}
