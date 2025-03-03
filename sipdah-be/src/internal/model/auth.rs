use crate::internal::model::error::Error;
use serde::{Deserialize, Serialize};
use validator::Validate;

pub trait Service {
    async fn sign_in(&self, req: &SignInRequest) -> Result<AuthResponse, Error>;

    async fn sign_up(&self, req: &SignUpRequest) -> Result<AuthResponse, Error>;

    async fn sign_out(&self) -> Result<(), Error>;

    async fn refresh(&self, req: &RefreshTokenRequest) -> Result<AuthResponse, Error>;

    fn verify_access_token(&self, token: &str) -> Result<Claim, Error>;

    fn verify_refresh_token(&self, token: &str) -> Result<Claim, Error>;
}

#[derive(Validate, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct SignInRequest {
    #[validate(
        email(message = "Invalid email format. Please provide a valid email address."),
        length(
            min = 1,
            max = 64,
            message = "Email length must be between 1 and 64 characters."
        )
    )]
    pub email: String,
    #[validate(length(
        min = 6,
        max = 16,
        message = "Password length must be between 6 and 16 characters."
    ))]
    pub password: String,
}

#[derive(Validate, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct SignUpRequest {
    #[validate(length(
        min = 1,
        max = 64,
        message = "Name length must be between 1 and 10 characters."
    ))]
    pub name: String,
    #[validate(
        email(message = "Invalid email format. Please provide a valid email address."),
        length(
            min = 1,
            max = 64,
            message = "Email length must be between 1 and 64 characters."
        )
    )]
    pub email: String,
    #[validate(length(
        min = 6,
        max = 16,
        message = "Password length must be between 6 and 16 characters."
    ))]
    pub password: String,
}

#[derive(Validate, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct RefreshTokenRequest {
    #[validate(length(min = 1, message = "Token is required"))]
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct AuthResponse {
    pub user_id: String,
    pub email: String,
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct Claim {
    pub(crate) sub: String,
    pub(crate) exp: i64,
    pub(crate) iat: i64,
    pub(crate) email: String,
}
