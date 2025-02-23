use crate::config::Config;
use crate::internal::common::uow::Uow;
use crate::internal::model::error::Error;
use crate::internal::model::role::Repository as RoleRepository;
use crate::internal::model::user::{
    Repository as UserRepository, Service as UserService, UserResponse,
};
use crate::internal::service::USER_ID;
use std::sync::Arc;

#[derive(Clone)]
pub struct Service<T1, T2, T3>
where
    T1: Uow + Send + Sync,
    T2: UserRepository + Send + Sync,
    T3: RoleRepository + Send + Sync,
{
    uow: Arc<T1>,
    user_repo: Arc<T2>,
    role_repo: Arc<T3>,
}

impl<T1, T2, T3> Service<T1, T2, T3>
where
    T1: Uow + Send + Sync,
    T2: UserRepository + Send + Sync,
    T3: RoleRepository + Send + Sync,
{
    pub fn new(uow: Arc<T1>, user_repo: Arc<T2>, role_repo: Arc<T3>) -> Self {
        Self {
            uow,
            user_repo,
            role_repo,
        }
    }
}

impl<T1, T2, T3> UserService for Service<T1, T2, T3>
where
    T1: Uow + Send + Sync,
    T2: UserRepository + Send + Sync,
    T3: RoleRepository + Send + Sync,
{
    async fn get_by_id(&self, user_id: &str) -> Result<UserResponse, Error> {
        let res = self
            .user_repo
            .find_by_id(user_id)
            .await?
            .map(|user| UserResponse {
                id: user.id,
                email: user.email,
                name: user.name,
                phone_number: user.phone_number,
                photo_url: user.photo_url,
                created_at: user.created_at,
                updated_at: user.updated_at,
            })
            .ok_or_else(|| Error::NotFound("User not found".to_string()))?;

        Ok(res)
    }

    async fn get_current(&self) -> Result<UserResponse, Error> {
        let (user_id, _) = USER_ID.get();
        self.get_by_id(&user_id).await
    }
}
