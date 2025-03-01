use crate::internal::common::response::Json as IntoJson;
use crate::internal::model::role;
use axum::{
    extract::{Path, State},
    response::IntoResponse
    ,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct RoleState<T1>
where
    T1: role::Service,
{
    pub role_service: Arc<T1>,
}

pub async fn create<T1: role::Service>(
    State(state): State<Arc<RoleState<T1>>>,
    axum::Json(payload): axum::Json<role::CreateRoleRequest>,
) -> impl IntoResponse + Send {
    state.role_service.create(&payload).await.json()
}

pub async fn get_by_id<T1: role::Service>(
    State(state): State<Arc<RoleState<T1>>>,
    Path(role_id): Path<String>,
) -> impl IntoResponse + Send {
    state.role_service.find_by_id(&role_id).await.json()
}

pub async fn get_all<T1: role::Service>(
    State(state): State<Arc<RoleState<T1>>>,
) -> impl IntoResponse + Send {
    state.role_service.find_all().await.json()
}
