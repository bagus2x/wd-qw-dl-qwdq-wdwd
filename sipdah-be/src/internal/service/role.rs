use crate::internal::common::id;
use crate::internal::model::error::Error;
use crate::internal::model::role::{
    CreateRoleRequest, Repository as RoleRepository, RoleResponse, Service as RoleService,
};
use chrono::Local;
use std::sync::Arc;
use validator::Validate;

#[derive(Clone)]
pub struct Service<T1>
where
    T1: RoleRepository,
{
    role_repo: Arc<T1>,
}

impl<T1> Service<T1>
where
    T1: RoleRepository,
{
    pub fn new(role_repo: Arc<T1>) -> Self {
        Self { role_repo }
    }
}

impl<T1> RoleService for Service<T1>
where
    T1: RoleRepository,
{
    async fn create(&self, role: &CreateRoleRequest) -> Result<(), Error> {
        role.validate()
            .map_err(|err| Error::BadRequest(err.to_string()))?;

        let exists = self.role_repo.exists_by_name(&role.name).await?;
        if exists {
            return Err(Error::Conflict(format!(
                "Role '{}' already exists",
                role.name
            )));
        }

        let new_role = crate::internal::model::role::Role {
            id: id::new(),
            name: role.name.clone(),
            created_at: Local::now(),
            updated_at: Local::now(),
        };

        self.role_repo.create(&new_role).await
    }

    async fn find_by_id(&self, role_id: &str) -> Result<RoleResponse, Error> {
        let role = self
            .role_repo
            .find_by_id(role_id)
            .await?
            .ok_or_else(|| Error::NotFound(format!("Role with ID '{}' not found", role_id)))?;

        Ok(RoleResponse {
            id: role.id,
            name: role.name,
            created_at: role.created_at,
            updated_at: role.updated_at,
        })
    }

    async fn find_all(&self) -> Result<Vec<RoleResponse>, Error> {
        let roles = self.role_repo.find_all().await?;
        let responses = roles
            .into_iter()
            .map(|role| RoleResponse {
                id: role.id,
                name: role.name,
                created_at: role.created_at,
                updated_at: role.updated_at,
            })
            .collect();

        Ok(responses)
    }
}
