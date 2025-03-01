use crate::internal::common::uow::Uow;
use crate::internal::model::error::Error;
use crate::internal::model::identity::get_current_identity;
use crate::internal::model::role::Repository as RoleRepository;
use crate::internal::model::user::{
    AddRolesRequest, Repository as UserRepository, Service as UserService, UserResponse,
};
use crate::internal::provider::cache::Cache as CacheProvider;
use std::sync::Arc;
use uow_macro::uow;

#[derive(Clone)]
pub struct Service<T1, T2, T3, T4>
where
    T1: Uow + Send + Sync,
    T2: UserRepository + Send + Sync,
    T3: RoleRepository + Send + Sync,
    T4: CacheProvider + Send + Sync,
{
    uow: Arc<T1>,
    user_repo: Arc<T2>,
    role_repo: Arc<T3>,
    cache_provider: Arc<T4>,
}

impl<T1, T2, T3, T4> Service<T1, T2, T3, T4>
where
    T1: Uow + Send + Sync,
    T2: UserRepository + Send + Sync,
    T3: RoleRepository + Send + Sync,
    T4: CacheProvider + Send + Sync,
{
    pub fn new(
        uow: Arc<T1>,
        user_repo: Arc<T2>,
        role_repo: Arc<T3>,
        cache_provider: Arc<T4>,
    ) -> Self {
        Self {
            uow,
            user_repo,
            role_repo,
            cache_provider,
        }
    }
}

impl<T1, T2, T3, T4> UserService for Service<T1, T2, T3, T4>
where
    T1: Uow + Send + Sync,
    T2: UserRepository + Send + Sync,
    T3: RoleRepository + Send + Sync,
    T4: CacheProvider + Send + Sync,
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

        self.cache_provider
            .set(format!("user:detail:{}", res.id), &res)
            .await?;

        Ok(res)
    }

    async fn get_current(&self) -> Result<UserResponse, Error> {
        let identity = get_current_identity()?;
        self.get_by_id(&identity.user_id).await
    }

    #[uow]
    async fn add_roles(&self, req: AddRolesRequest) -> Result<UserResponse, Error> {
        if self.user_repo.exists_by_id(&req.user_id).await? {
            return Err(Error::NotFound("User is not found".to_string()));
        }

        for role in req.roles {
            self.role_repo.add(&req.user_id, &role).await?;
        }

        self.get_by_id(&req.user_id).await
    }
}
