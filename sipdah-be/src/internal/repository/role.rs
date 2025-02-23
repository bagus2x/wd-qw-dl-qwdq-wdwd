use crate::internal::common::uow::{get_transaction, CURRENT_TRANSACTION};
use crate::internal::model;
use crate::internal::model::error::Error;
use crate::internal::model::role::Role;
use sqlx::{MySql, Pool};
use std::sync::Arc;

#[derive(Clone)]
pub struct Repository {
    pool: Arc<Pool<MySql>>,
}

impl Repository {
    pub fn new(pool: Arc<Pool<MySql>>) -> Self {
        Self { pool }
    }
}

impl model::role::Repository for Repository {
    async fn create(&self, role: &Role) -> Result<(), Error> {
        let query = r#"
            INSERT INTO role (id, name, created_at, updated_at)
            VALUES (UUID_TO_BIN(?), ?, ?, ?)
        "#;

        let tx = get_transaction()?;

        sqlx::query(query)
            .bind(&role.id)
            .bind(&role.name)
            .bind(role.created_at)
            .bind(role.updated_at)
            .execute(&mut **tx)
            .await
            .map_err(|err| Error::Internal(err.to_string()))?;

        CURRENT_TRANSACTION
            .try_with(|tx1| *tx1.borrow_mut() = Some(tx))
            .map_err(|err| Error::Internal(err.to_string()))?;

        Ok(())
    }

    async fn find_by_id(&self, role_id: &str) -> Result<Role, Error> {
        let query = r#"
            SELECT BIN_TO_UUID(id) as id, name, created_at, updated_at, deleted_at
            FROM role
            WHERE id = UUID_TO_BIN(?)
        "#;

        let row = sqlx::query_as::<_, Role>(query)
            .bind(role_id)
            .fetch_one(&*self.pool)
            .await
            .map_err(|err| Error::NotFound(format!("Role not found: {}", err)))?;

        Ok(row)
    }

    async fn find_by_name(&self, name: &str) -> Result<Role, Error> {
        let query = r#"
            SELECT BIN_TO_UUID(id) as id, name, created_at, updated_at, deleted_at
            FROM role
            WHERE name = ?
        "#;

        let row = sqlx::query_as::<_, Role>(query)
            .bind(name)
            .fetch_one(&*self.pool)
            .await
            .map_err(|err| Error::NotFound(format!("Role not found: {}", err)))?;

        Ok(row)
    }

    async fn exists_by_name(&self, name: &str) -> Result<bool, Error> {
        let query = r#"
            SELECT EXISTS(SELECT 1 FROM role WHERE name = ?)
        "#;

        let exists: (bool,) = sqlx::query_as(query)
            .bind(name)
            .fetch_one(&*self.pool)
            .await
            .map_err(|err| Error::Internal(err.to_string()))?;

        Ok(exists.0)
    }

    async fn add(&self, user_id: &str, role_id: &str) -> Result<(), Error> {
        let query = r#"
            INSERT INTO user_role (user_id, role_id)
            VALUES (UUID_TO_BIN(?), UUID_TO_BIN(?))
        "#;

        let tx = get_transaction()?;

        sqlx::query(query)
            .bind(user_id)
            .bind(role_id)
            .execute(&mut **tx)
            .await
            .map_err(|err| Error::Internal(err.to_string()))?;

        CURRENT_TRANSACTION
            .try_with(|tx1| {
                *tx1.borrow_mut() = Some(tx);
            })
            .map_err(|err| Error::Internal(err.to_string()))?;

        Ok(())
    }
}
