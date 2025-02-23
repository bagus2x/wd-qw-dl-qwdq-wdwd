use crate::internal::model::error::Error;
use crate::internal::model::user::User;
use sqlx::mysql::MySqlArguments;
use sqlx::query::{Query, QueryAs};
use sqlx::{MySql, MySqlPool, Pool, Transaction};
use std::cell::RefCell;
use std::future::Future;
use std::sync::Arc;
use tokio::task_local;

// Task-local storage now holds a reference
task_local! {
    pub static CURRENT_TRANSACTION: RefCell<Option<&'static mut Transaction<'static, MySql>>>;
}

// Transaction manager that uses task-local storage
pub struct TransactionManager {
    pool: Arc<MySqlPool>,
}

pub trait Uow {
    async fn run<F, R>(&self, operation: F) -> Result<R, Error>
    where
        F: Future<Output = Result<R, Error>>;
}

impl TransactionManager {
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        Self { pool }
    }
}

impl Uow for TransactionManager {
    async fn run<F, R>(&self, operation: F) -> Result<R, Error>
    where
        F: Future<Output = Result<R, Error>>,
    {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|err| Error::Internal(err.to_string()))?;
        // Safety: we control the transaction lifetime within this scope
        let tx_static = unsafe { std::mem::transmute(&mut tx) };

        let result = CURRENT_TRANSACTION
            .scope(RefCell::new(Some(tx_static)), operation)
            .await;

        match result {
            Ok(value) => {
                tx.commit()
                    .await
                    .map_err(|err| Error::Internal(err.to_string()))?;
                Ok(value)
            }
            Err(e) => {
                tx.rollback()
                    .await
                    .map_err(|err| Error::Internal(err.to_string()))?;
                Err(e)
            }
        }
    }
}

// Helper function to get current transaction
pub fn get_transaction() -> Result<&'static mut Transaction<'static, MySql>, Error> {
    CURRENT_TRANSACTION
        .try_with(|tx| {
            tx.borrow_mut()
                .take()
                .ok_or(Error::Internal("No active transaction".into()))
        })
        .map_err(|_| Error::Internal("Cannot access transaction".into()))?
}

#[macro_export]
macro_rules! with_transaction {
    ($tx:expr, $body:expr) => {
        $body;

        CURRENT_TRANSACTION
            .try_with(|tx1| {
                *tx1.borrow_mut() = Some($tx);
            })
            .map_err(|err| Error::Internal(err.to_string()))?;
    };
}

pub async fn execute(
    query: Query<'_, MySql, MySqlArguments>,
    pool: &Pool<MySql>,
) -> Result<(), Error> {
    let tx = get_transaction();

    match tx {
        Ok(tx) => {
            with_transaction!(tx, {
                query
                    .execute(&mut **tx)
                    .await
                    .map_err(|err| Error::Internal(err.to_string()))?
            });
        }
        Err(_) => {
            query
                .execute(pool)
                .await
                .map_err(|err| Error::Internal(err.to_string()))?;
        }
    };

    Ok(())
}

pub async fn fetch_one_as(
    query: QueryAs<'_, MySql, User, MySqlArguments>,
    pool: &Pool<MySql>,
) -> Result<Option<User>, Error> {
    let tx = get_transaction();

    match tx {
        Ok(tx) => {
            with_transaction!(tx, {
                query
                    .fetch_one(&mut **tx)
                    .await
                    .map_err(|err| Error::Internal(err.to_string()))?
            });
        }
        Err(_) => {
            query
                .fetch_one(pool)
                .await
                .map_err(|err| Error::Internal(err.to_string()))?;
        }
    };

    Ok(None)
}
