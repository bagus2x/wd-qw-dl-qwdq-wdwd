use crate::internal::common::uow;
use crate::internal::model;
use crate::internal::model::error::Error;
use crate::internal::model::user::User;
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

impl model::user::Repository for Repository {
    async fn create(&self, user: &User) -> Result<(), Error> {
        let sql = r#"
            INSERT INTO
                user (id, email, password, name, phone_number, photo_url, created_at, updated_at)
            VALUES
                (UUID_TO_BIN(?), ?, ?, ?, ?, ?, ?, ?)
        "#;

        let query = sqlx::query(sql)
            .bind(&user.id)
            .bind(&user.email)
            .bind(&user.password)
            .bind(&user.name)
            .bind(&user.phone_number)
            .bind(&user.photo_url)
            .bind(user.created_at)
            .bind(user.updated_at);

        Ok(uow::execute(query, &*self.pool).await?)
    }

    async fn find_by_id(&self, user_id: &str) -> Result<Option<User>, Error> {
        let sql = r#"
            SELECT
                BIN_TO_UUID(id) as id, email, password, name, phone_number, photo_url, created_at,
                updated_at, deleted_at
            FROM
                user
            WHERE
                id = UUID_TO_BIN(?)
        "#;

        let query = sqlx::query_as::<_, User>(sql).bind(user_id);
        let user = uow::fetch_one_as(query, &*self.pool).await?;

        Ok(user)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error> {
        let sql = r#"
            SELECT
                BIN_TO_UUID(id) as id, email, password, name, phone_number, photo_url, created_at,
                updated_at, deleted_at
            FROM
                user
            WHERE
                email = ?
        "#;

        let query = sqlx::query_as::<_, User>(sql).bind(email);
        let user = uow::fetch_one_as(query, &*self.pool).await?;

        Ok(user)
    }

    async fn exists_by_email(&self, email: &str) -> Result<bool, Error> {
        let sql = r#"
            SELECT EXISTS(SELECT 1 FROM user WHERE email = ?)
        "#;

        let query = sqlx::query_as(sql).bind(email);
        let exists: (bool,) = uow::fetch_one(query, &*self.pool).await?;

        Ok(exists.0)
    }
}
