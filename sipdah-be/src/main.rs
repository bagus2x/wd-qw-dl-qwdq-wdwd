use crate::config::Config;
use crate::db::mysql;
use crate::db::redis;
use crate::internal::common::uow;
use crate::internal::router::auth;
use crate::internal::router::role;
use crate::internal::router::user;
use crate::internal::{middleware, provider, repository, service};
use axum::http::{header, HeaderValue, Method};
use axum::middleware::from_fn_with_state;
use axum::routing::{delete, get, patch, post};
use axum::Router;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing::{error, Level};

mod config;
mod db;
mod internal;

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

    let cache_provider = Arc::new(provider::cache::Redis::new(
        Arc::clone(&config),
        Arc::clone(&redis),
    ));

    let uow = Arc::new(uow::TransactionManager::new(Arc::clone(&mysql)));

    let auth_service = Arc::new(service::auth::Service::new(
        Arc::clone(&config),
        Arc::clone(&uow),
        Arc::clone(&user_repo),
        Arc::clone(&role_repo),
        Arc::clone(&cache_provider),
    ));
    let user_service = Arc::new(service::user::Service::new(
        Arc::clone(&uow),
        Arc::clone(&user_repo),
        Arc::clone(&role_repo),
        Arc::clone(&cache_provider),
    ));
    let role_service = Arc::new(service::role::Service::new(Arc::clone(&role_repo)));

    let auth_state = Arc::new(auth::AuthState {
        auth_service: Arc::clone(&auth_service),
    });
    let user_state = Arc::new(user::UserState {
        user_service: Arc::clone(&user_service),
    });
    let role_state = Arc::new(role::RoleState {
        role_service: Arc::clone(&role_service),
    });

    let auth_route = Router::new()
        .route("/api/v1/auth/signup", post(auth::sign_up))
        .route("/api/v1/auth/signin", post(auth::sign_in))
        .route("/api/v1/auth/refresh", get(auth::refresh))
        .merge(
            Router::new()
                .route("/api/v1/auth/signout", delete(auth::sign_out))
                .route_layer(from_fn_with_state(
                    Arc::clone(&auth_state),
                    middleware::auth,
                )),
        )
        .with_state(Arc::clone(&auth_state));

    let user_route = Router::new()
        .route("/api/v1/user", get(user::get_current))
        .route("/api/v1/user/{user_id}", get(user::get_by_id))
        .route("/api/v1/user/role", patch(user::add_roles))
        .route_layer(from_fn_with_state(
            Arc::clone(&auth_state),
            middleware::auth,
        ))
        .with_state(Arc::clone(&user_state));

    let role_route = Router::new()
        .route("/api/v1/roles", post(role::create))
        .route("/api/v1/roles", get(role::get_all))
        .route("/api/v1/roles/{role_id}", get(role::get_by_id))
        .route_layer(from_fn_with_state(
            Arc::clone(&auth_state),
            middleware::auth,
        ))
        .with_state(Arc::clone(&role_state));

    let allowed_origins: Vec<HeaderValue> = config
        .cors_allowed_origins
        .iter()
        .map(|origin| HeaderValue::from_str(origin).unwrap())
        .collect();

    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_origin(allowed_origins)
        .allow_headers([
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::ACCEPT,
            header::COOKIE,
        ])
        .allow_credentials(true);

    let app = Router::new()
        .merge(auth_route)
        .merge(user_route)
        .merge(role_route)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
