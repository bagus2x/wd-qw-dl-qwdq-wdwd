use crate::internal::model::error::Error;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

pub const ROLE_USER: &str = "USER";

#[derive(FromRow)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>
}

pub trait Repository {
    async fn create(&self, role: &Role) -> Result<(), Error>;

    async fn find_by_id(&self, role_id: &str) -> Result<Option<Role>, Error>;

    async fn find_all(&self) -> Result<Vec<Role>, Error>;

    async fn find_by_name(&self, name: &str) -> Result<Option<Role>, Error>;

    async fn exists_by_name(&self, name: &str) -> Result<bool, Error>;

    async fn add(&self, user_id: &str, role_id: &str) -> Result<(), Error>;
}

pub trait Service {
    async fn create(&self, role: &CreateRoleRequest) -> Result<(), Error>;

    async fn find_by_id(&self, role_id: &str) -> Result<RoleResponse, Error>;

    async fn find_all(&self) -> Result<Vec<RoleResponse>, Error>;
}

#[derive(Validate, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct CreateRoleRequest {
    pub name: String,
}

#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct RoleResponse {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}
