use crate::internal::model::auth;
use crate::internal::model::auth::{SignInRequest, SignUpRequest};
use crate::internal::common::response::Json as IntoJson;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use std::sync::Arc;

#[derive(Clone)]
pub struct AuthState<T1>
where
    T1: auth::Service,
{
    pub auth_service: Arc<T1>,
}

pub async fn sign_up<T1: auth::Service>(
    State(state): State<Arc<AuthState<T1>>>,
    Json(req): Json<SignUpRequest>,
) -> impl IntoResponse + Send {
    state.auth_service.sign_up(&req).await.json()
}

pub async fn sign_in<T1: auth::Service>(
    State(state): State<Arc<AuthState<T1>>>,
    Json(req): Json<SignInRequest>,
) -> impl IntoResponse + Send {
    state.auth_service.sign_in(&req).await.json()
}
