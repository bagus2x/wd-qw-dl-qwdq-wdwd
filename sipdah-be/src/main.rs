use crate::config::Config;
use crate::db::mysql;
use crate::db::redis;
use crate::internal::common::uow;
use crate::internal::router::auth::{refresh, sign_in, sign_out, sign_up, AuthState};
use crate::internal::router::user::{get_by_id, get_current, UserState};
use crate::internal::{middleware, repository, service};
use axum::http::{header, Method};
use axum::middleware::from_fn_with_state;
use axum::routing::{delete, get, post};
use axum::Router;
use std::sync::Arc;
use tokio::task_local;
use tower_http::cors::{Any, CorsLayer};
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
    let mysql = match mysql::Database::new(Arc::clone(&config)).await {
        Ok(db) => Arc::new(db.pool),
        Err(err) => {
            error!(error = %err, "Failed to initialize mysql");
            return;
        }
    };
    let redis = match redis::Database::new(Arc::clone(&config)) {
        Ok(db) => Arc::new(db.pool),
        Err(err) => {
            error!(error = %err, "Failed to initialize redis");
            return;
        }
    };

    let user_repo = Arc::new(repository::user::Repository::new(Arc::clone(&mysql)));
    let role_repo = Arc::new(repository::role::Repository::new(Arc::clone(&mysql)));
    let cache_repo = Arc::new(repository::cache::Repository::new(
        Arc::clone(&config),
        Arc::clone(&redis),
    ));

    let uow = Arc::new(uow::TransactionManager::new(Arc::clone(&mysql)));

    let auth_service = Arc::new(service::auth::Service::new(
        Arc::clone(&config),
        Arc::clone(&uow),
        Arc::clone(&user_repo),
        Arc::clone(&role_repo),
        Arc::clone(&cache_repo),
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

    let open_router = Router::new()
        .route("/api/v1/auth/signup", post(sign_up))
        .route("/api/v1/auth/signin", post(sign_in))
        .route("/api/v1/auth/refresh", get(refresh))
        .with_state(Arc::clone(&auth_state));

    let protected_router = Router::new()
        .route("/api/v1/auth/signout", delete(sign_out))
        .route("/api/v1/user", get(get_current))
        .route("/api/v1/user/{user_id}", get(get_by_id))
        .route_layer(from_fn_with_state(Arc::clone(&auth_state), middleware::auth))
        .with_state(Arc::clone(&user_state))
        .with_state(Arc::clone(&auth_state));

    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::PUT,
            Method::OPTIONS,
        ])
        .allow_origin(["http://localhost:5173".parse().unwrap()])
        .allow_headers([
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::ACCEPT,
            header::COOKIE,
        ])
        .allow_credentials(true);

    let app = Router::new()
        .merge(open_router)
        .merge(protected_router)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
