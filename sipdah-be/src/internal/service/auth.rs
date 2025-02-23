use crate::config::Config;
use crate::internal::common::id;
use crate::internal::common::uow::Uow;
use crate::internal::model::auth::{
    AuthResponse, Claim, Service as AuthService, SignInRequest, SignUpRequest,
};
use crate::internal::model::error::Error;
use crate::internal::model::role::{Repository as RoleRepository, ROLE_USER};
use crate::internal::model::user::{Repository as UserRepository, User};
use chrono::Local;
use jsonwebtoken::Algorithm;
use std::ops::Add;
use std::sync::Arc;
use tracing::info;
use validator::Validate;

#[derive(Clone)]
pub struct Service<T1, T2, T3>
where
    T1: Uow,
    T2: UserRepository,
    T3: RoleRepository,
{
    config: Arc<Config>,
    uow: Arc<T1>,
    user_repo: Arc<T2>,
    role_repo: Arc<T3>,
}

impl<T1, T2, T3> Service<T1, T2, T3>
where
    T1: Uow,
    T2: UserRepository,
    T3: RoleRepository,
{
    pub fn new(config: Arc<Config>, uow: Arc<T1>, user_repo: Arc<T2>, role_repo: Arc<T3>) -> Self {
        Self {
            config,
            uow,
            user_repo,
            role_repo,
        }
    }

    fn create_token(
        &self,
        key: &[u8],
        user_id: &str,
        email: &str,
        exp: i64,
        iat: i64,
    ) -> Result<String, Error> {
        let claims = serde_json::json!(Claim {
            sub: user_id.to_string(),
            username: String::from(email),
            exp,
            iat,
        });

        jsonwebtoken::encode(
            &jsonwebtoken::Header::new(Algorithm::HS256),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(key),
        )
        .map_err(|e| Error::Internal(e.to_string()))
    }
}

impl<T1, T2, T3> AuthService for Service<T1, T2, T3>
where
    T1: Uow + Send + Sync,
    T2: UserRepository + Send + Sync,
    T3: RoleRepository + Send + Sync,
{
    async fn sign_in(&self, req: &SignInRequest) -> Result<AuthResponse, Error> {
        todo!()
    }

    async fn sign_up(&self, req: &SignUpRequest) -> Result<AuthResponse, Error> {
        self.uow
            .run(async {
                req.validate()
                    .map_err(|err| Error::BadRequest(err.to_string()))?;

                let is_present = self.user_repo.exists_by_email(&req.email).await?;
                if is_present {
                    return Err(Error::BadRequest(format!(
                        "Email {} already exists",
                        req.email
                    )));
                }

                let password = bcrypt::hash(&req.password, 12)
                    .map_err(|err| Error::Internal(err.to_string()))?;

                let user = User {
                    id: id::new(),
                    email: req.email.clone(),
                    password,
                    name: req.name.clone(),
                    phone_number: None,
                    photo_url: None,
                    created_at: Local::now(),
                    updated_at: Local::now(),
                    deleted_at: None,
                };

                self.user_repo.create(&user).await?;

                let role = self.role_repo.find_by_name(ROLE_USER).await?;

                self.role_repo.add(&user.id, &role.id).await?;

                let access_token = self.create_token(
                    self.config.access_token_key_secret.as_ref(),
                    &user.id,
                    &user.email,
                    chrono::Utc::now()
                        .add(chrono::Duration::minutes(10))
                        .timestamp(),
                    chrono::Utc::now().timestamp(),
                )?;
                let refresh_token = self.create_token(
                    self.config.refresh_token_key_secret.as_ref(),
                    &user.id,
                    &user.email,
                    chrono::Utc::now()
                        .add(chrono::Duration::days(7))
                        .timestamp(),
                    chrono::Utc::now().timestamp(),
                )?;

                Ok(AuthResponse {
                    user_id: user.id,
                    email: user.email,
                    access_token,
                    refresh_token,
                })
            })
            .await
    }
}
