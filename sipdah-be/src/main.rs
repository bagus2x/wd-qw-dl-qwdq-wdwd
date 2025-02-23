use crate::config::Config;
use crate::db::mysql::MysqlDatabase;
use crate::internal::common::uow;
use crate::internal::handler::into::Json as IntoJson;
use crate::internal::model::auth::SignUpRequest;
use crate::internal::model::{auth, role, user};
use crate::internal::{repository, service};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use sqlx::{MySql, Pool};
use std::sync::Arc;
use tokio::task_local;
use tracing::{error, info, Level};

mod config;
mod db;
mod internal;

#[derive(Clone)]
struct AppState<T1, T2, T3, T4>
where
    T1: uow::Uow,
    T2: user::Repository,
    T3: role::Repository,
    T4: auth::Service,
{
    pub config: Arc<Config>,
    pub db: Arc<Pool<MySql>>,
    pub uow: Arc<T1>,
    pub user_repo: Arc<T2>,
    pub role_repo: Arc<T3>,
    pub auth_service: Arc<T4>,
}

task_local! {
    pub static  A: Option<i32>
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG) // Set the maximum log level
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

    let state = AppState {
        config: Arc::clone(&config),
        db,
        uow,
        user_repo,
        role_repo,
        auth_service,
    };

    let app = Router::new()
        .route("/auth/signup", post(signup))
        .route("/auth/signin", get(signin))
        .with_state(state.into());

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn signup<T1: uow::Uow, T2: user::Repository, T3: role::Repository, T4: auth::Service>(
    State(state): State<Arc<AppState<T1, T2, T3, T4>>>,
    Json(req): Json<SignUpRequest>,
) -> impl IntoResponse + Send {
    state.auth_service.sign_up(&req).await.json()
}

async fn signin<T1: uow::Uow, T2: user::Repository, T3: role::Repository, T4: auth::Service>(
    State(state): State<Arc<AppState<T1, T2, T3, T4>>>,
) -> impl IntoResponse {
    "hello"
}
