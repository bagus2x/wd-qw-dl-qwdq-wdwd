use crate::internal;
use crate::internal::common::response;
use crate::internal::model::error::Error;
use crate::internal::model::identity::{Identity, IDENTITY};
use crate::internal::router::auth::AuthState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};
use internal::model::auth;
use response::json_error;
use std::sync::Arc;

pub async fn auth<T1: auth::Service>(
    State(state): State<Arc<AuthState<T1>>>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    if auth_header.is_none() {
        let error = Error::Unauthorized("Token is not found".to_string());
        return Ok(json_error::<String>(error).into_response());
    }

    let token = auth_header.unwrap();
    if token.len() < 7 {
        let error = Error::Unauthorized("Token is incorrect".to_string());
        return Ok(json_error::<String>(error).into_response());
    }

    match state.auth_service.verify_token(&token[7..]) {
        Ok(claim) => {
            let identity = Identity {
                user_id: claim.sub,
                email: claim.email,
            };
            Ok(IDENTITY.scope(identity, next.run(req)).await)
        }
        Err(error) => Ok(json_error::<String>(error).into_response()),
    }
}
