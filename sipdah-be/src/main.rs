use crate::config::Config;
use crate::db::mysql::MysqlDatabase;
use crate::internal::common::uow;
use crate::internal::router::auth::{sign_in, sign_up, AuthState};
use crate::internal::router::user::{get_by_id, get_current, UserState};
use crate::internal::{middleware, repository, service};
use axum::middleware::from_fn_with_state;
use axum::routing::{get, post};
use axum::Router;
use std::sync::Arc;
use tokio::task_local;
use tracing::{error, Level};

mod config;
mod db;
mod internal;

task_local! {
    pub static  A: Option<i32>
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let config = Arc::new(Config::new());
    let db = match MysqlDatabase::new(Arc::clone(&config)).await {
        Ok(db) => Arc::new(db.pool),
        Err(err) => {
            error!(error = %err, "Failed to initialize database");
            return;
        }
    };

    let user_repo = Arc::new(repository::user::Repository::new(Arc::clone(&db)));
    let role_repo = Arc::new(repository::role::Repository::new(Arc::clone(&db)));

    let uow = Arc::new(uow::TransactionManager::new(Arc::clone(&db)));

    let auth_service = Arc::new(service::auth::Service::new(
        Arc::clone(&config),
        Arc::clone(&uow),
        Arc::clone(&user_repo),
        Arc::clone(&role_repo),
    ));
    let user_service = Arc::new(service::user::Service::new(
        Arc::clone(&uow),
        Arc::clone(&user_repo),
        Arc::clone(&role_repo),
    ));

    let auth_state = Arc::new(AuthState {
        auth_service: Arc::clone(&auth_service),
    });
    let user_state = Arc::new(UserState {
        user_service: Arc::clone(&user_service),
    });

    let auth_router = Router::new()
        .route("/api/v1/auth/signup", post(sign_up))
        .route("/api/v1/auth/signin", post(sign_in))
        .with_state(Arc::clone(&auth_state));

    let user_router = Router::new()
        .route("/api/v1/user", get(get_current))
        .route("/api/v1/user/{user_id}", get(get_by_id))
        .route_layer(from_fn_with_state(auth_state, middleware::auth))
        .with_state(Arc::clone(&user_state));

    let app = Router::new().merge(auth_router).merge(user_router);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
