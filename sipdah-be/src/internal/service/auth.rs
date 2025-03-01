use crate::config::Config;
use crate::internal::common::id;
use crate::internal::common::uow::Uow;
use crate::internal::model::auth::{
    AuthResponse, Claim, RefreshTokenRequest, Service as AuthService, SignInRequest, SignUpRequest,
};
use crate::internal::model::error::Error;
use crate::internal::model::identity::get_current_identity;
use crate::internal::model::role::{Repository as RoleRepository, ROLE_USER};
use crate::internal::model::user::{Repository as UserRepository, User};
use crate::internal::provider::cache::Cache as CacheProvider;
use chrono::Local;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use std::ops::Add;
use std::sync::Arc;
use tracing::info;
use uow_macro::uow;
use validator::Validate;

#[derive(Clone)]
pub struct Service<T1, T2, T3, T4>
where
    T1: Uow,
    T2: UserRepository,
    T3: RoleRepository,
    T4: CacheProvider,
{
    config: Arc<Config>,
    uow: Arc<T1>,
    user_repo: Arc<T2>,
    role_repo: Arc<T3>,
    cache_repo: Arc<T4>,
}

impl<T1, T2, T3, T4> Service<T1, T2, T3, T4>
where
    T1: Uow,
    T2: UserRepository,
    T3: RoleRepository,
    T4: CacheProvider,
{
    pub fn new(
        config: Arc<Config>,
        uow: Arc<T1>,
        user_repo: Arc<T2>,
        role_repo: Arc<T3>,
        cache_repo: Arc<T4>,
    ) -> Self {
        Self {
            config,
            uow,
            user_repo,
            role_repo,
            cache_repo,
        }
    }

    async fn issue_tokens(&self, user: &User) -> Result<AuthResponse, Error> {
        let access_token = self.create_token(
            self.config.access_token_key_secret.as_ref(),
            &user.id,
            &user.email,
            chrono::Utc::now()
                .add(self.config.access_token_key_ttl)
                .timestamp(),
            chrono::Utc::now().timestamp(),
        )?;
        let refresh_token = self.create_token(
            self.config.refresh_token_key_secret.as_ref(),
            &user.id,
            &user.email,
            chrono::Utc::now()
                .add(self.config.refresh_token_key_ttl)
                .timestamp(),
            chrono::Utc::now().timestamp(),
        )?;
        let key = format!("auth:refresh-token:{}", user.id);

        self.cache_repo
            .setx(key, &refresh_token, self.config.refresh_token_key_ttl)
            .await?;

        Ok(AuthResponse {
            user_id: user.id.clone(),
            email: user.email.clone(),
            refresh_token,
            access_token,
        })
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
            email: String::from(email),
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

impl<T1, T2, T3, T4> AuthService for Service<T1, T2, T3, T4>
where
    T1: Uow + Send + Sync,
    T2: UserRepository + Send + Sync,
    T3: RoleRepository + Send + Sync,
    T4: CacheProvider + Send + Sync,
{
    async fn sign_in(&self, req: &SignInRequest) -> Result<AuthResponse, Error> {
        req.validate()
            .map_err(|err| Error::BadRequest(err.to_string()))?;

        let user = self
            .user_repo
            .find_by_email(&req.email)
            .await?
            .ok_or_else(|| {
                Error::NotFound(format!("User with email {} is not found", req.email))
            })?;

        let is_matching = bcrypt::verify(&req.password, &user.password)
            .map_err(|e| Error::Internal(e.to_string()))?;
        if !is_matching {
            return Err(Error::BadRequest("Password doesn't match".to_string()));
        }

        self.issue_tokens(&user).await
    }

    #[uow]
    async fn sign_up(&self, req: &SignUpRequest) -> Result<AuthResponse, Error> {
        req.validate()
            .map_err(|err| Error::BadRequest(err.to_string()))?;

        let is_present = self.user_repo.exists_by_email(&req.email).await?;
        if is_present {
            return Err(Error::Conflict(format!(
                "Email {} already exists",
                req.email
            )));
        }

        let password =
            bcrypt::hash(&req.password, 10).map_err(|err| Error::Internal(err.to_string()))?;

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

        let role = self
            .role_repo
            .find_by_name(ROLE_USER)
            .await?
            .ok_or_else(|| Error::NotFound(format!("Role {} is not found", ROLE_USER)))?;

        self.role_repo.add(&user.id, &role.id).await?;

        self.issue_tokens(&user).await
    }

    async fn sign_out(&self) -> Result<(), Error> {
        let identity = get_current_identity()?;
        let key = format!("auth:refresh-token:{}", identity.user_id);

        info!("Signout for user {}", identity.email);

        self.cache_repo.del(key).await
    }

    async fn refresh(&self, req: &RefreshTokenRequest) -> Result<AuthResponse, Error> {
        req.validate()
            .map_err(|err| Error::BadRequest(err.to_string()))?;

        let claim = self.verify_refresh_token(&req.refresh_token)?;
        let key = format!("auth:refresh-token:{}", claim.sub);

        match self.cache_repo.get::<String>(key).await? {
            Some(token) => {
                if token != req.refresh_token {
                    return Err(Error::BadRequest("Token doesn't match".to_string()));
                }

                let user = self
                    .user_repo
                    .find_by_id(&claim.sub)
                    .await?
                    .ok_or_else(|| Error::NotFound("User not found".to_string()))?;

                self.issue_tokens(&user).await
            }
            None => Err(Error::NotFound("Token is not found".to_string())),
        }
    }

    fn verify_access_token(&self, token: &str) -> Result<Claim, Error> {
        match jsonwebtoken::decode::<Claim>(
            &token,
            &DecodingKey::from_secret(self.config.access_token_key_secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        ) {
            Ok(data) => Ok(data.claims),
            Err(error) => match error.kind() {
                ErrorKind::ExpiredSignature => {
                    Err(Error::Unauthorized("Token is expired".to_string()))
                }
                ErrorKind::InvalidSignature => {
                    Err(Error::BadRequest("Token is not valid".to_string()))
                }
                _ => Err(Error::Internal(error.to_string())),
            },
        }
    }

    fn verify_refresh_token(&self, token: &str) -> Result<Claim, Error> {
        match jsonwebtoken::decode::<Claim>(
            &token,
            &DecodingKey::from_secret(self.config.refresh_token_key_secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        ) {
            Ok(data) => Ok(data.claims),
            Err(error) => match error.kind() {
                ErrorKind::ExpiredSignature => {
                    Err(Error::BadRequest("Token is expired".to_string()))
                }
                ErrorKind::InvalidSignature => {
                    Err(Error::BadRequest("Token is not valid".to_string()))
                }
                _ => Err(Error::Internal(error.to_string())),
            },
        }
    }
}
