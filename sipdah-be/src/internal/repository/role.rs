use crate::internal::common::uow;
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
        let sql = r#"
            INSERT INTO 
                role (id, name, created_at, updated_at)
            VALUES 
                (UUID_TO_BIN(?), ?, ?, ?)
        "#;

        let query = sqlx::query(sql)
            .bind(&role.id)
            .bind(&role.name)
            .bind(role.created_at)
            .bind(role.updated_at);

        Ok(uow::execute(query, &*self.pool).await?)
    }

    async fn find_by_id(&self, role_id: &str) -> Result<Option<Role>, Error> {
        let sql = r#"
            SELECT
                BIN_TO_UUID(id) as id, name, created_at, updated_at, deleted_at
            FROM
                role
            WHERE
                id = UUID_TO_BIN(?)
        "#;

        let query = sqlx::query_as::<_, Role>(sql).bind(role_id);
        let role = uow::fetch_one_as(query, &*self.pool).await?;

        Ok(role)
    }

    async fn find_all(&self) -> Result<Vec<Role>, Error> {
        todo!()
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<Role>, Error> {
        let sql = r#"
            SELECT 
                BIN_TO_UUID(id) as id, name, created_at, updated_at, deleted_at
            FROM 
                role
            WHERE 
                name = ?
        "#;

        let query = sqlx::query_as::<_, Role>(sql).bind(name);
        let role = uow::fetch_one_as(query, &*self.pool).await?;

        Ok(role)
    }

    async fn exists_by_name(&self, name: &str) -> Result<bool, Error> {
        let sql = r#"
            SELECT EXISTS(SELECT 1 FROM role WHERE name = ?)
        "#;

        let query = sqlx::query_as(sql).bind(name);
        let exists: (bool,) = uow::fetch_one(query, &*self.pool).await?;

        Ok(exists.0)
    }

    async fn add(&self, user_id: &str, role_id: &str) -> Result<(), Error> {
        let sql = r#"
            INSERT INTO user_role (user_id, role_id)
            VALUES (UUID_TO_BIN(?), UUID_TO_BIN(?))
        "#;

        let query = sqlx::query(sql).bind(user_id).bind(role_id);

        Ok(uow::execute(query, &*self.pool).await?)
    }
}
