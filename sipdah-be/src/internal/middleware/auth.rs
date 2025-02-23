use crate::internal;
use crate::internal::router::auth::AuthState;
use crate::internal::service::USER_ID;
use axum::extract::State;
use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};
use internal::model::auth;
use std::sync::Arc;

pub async fn auth<T1: auth::Service>(
    State(state): State<Arc<AuthState<T1>>>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    if auth_header.len() < 7 {
        return Err(StatusCode::UNAUTHORIZED);
    }

    if let Ok(claim) = state.auth_service.verify_token(&auth_header[7..]) {
        let auth = (claim.sub, claim.email);

        Ok(USER_ID.scope(auth, next.run(req)).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
