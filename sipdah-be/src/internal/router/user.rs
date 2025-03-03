use crate::internal::common::response::Json as IntoJson;
use crate::internal::model::user;
use crate::internal::model::user::AddRolesRequest;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserState<T1>
where
    T1: user::Service,
{
    pub user_service: Arc<T1>,
}

pub async fn get_by_id<T1: user::Service>(
    State(state): State<Arc<UserState<T1>>>,
    Path(user_id): Path<String>,
) -> impl IntoResponse + Send {
    state.user_service.get_by_id(&user_id).await.json()
}

pub async fn get_current<T1: user::Service>(
    State(state): State<Arc<UserState<T1>>>,
) -> impl IntoResponse + Send {
    state.user_service.get_current().await.json()
}

pub async fn add_roles<T1: user::Service>(
    State(state): State<Arc<UserState<T1>>>,
    Path(user_id): Path<String>,
    Json(mut req): Json<AddRolesRequest>,
) -> impl IntoResponse + Send {
    req.user_id = user_id;
    state.user_service.add_roles(req).await.json()
}
