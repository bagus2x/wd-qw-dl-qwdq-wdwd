use crate::internal;
use crate::internal::common::response;
use crate::internal::model::error::Error;
use crate::internal::model::identity::{Identity, IDENTITY};
use crate::internal::router::auth::AuthState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use axum_extra::extract::CookieJar;
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use internal::model::auth;
use response::json_error;
use std::sync::Arc;

pub async fn auth<T1: auth::Service>(
    State(state): State<Arc<AuthState<T1>>>,
    bearer: Option<TypedHeader<Authorization<Bearer>>>,
    jar: CookieJar,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = bearer
        .as_ref()
        .map(|b| b.token().to_string())
        .or_else(|| jar.get("access_token").map(|c| c.value().to_string()));

    let token = match token {
        Some(t) => t,
        None => {
            let error = Error::BadRequest("Access token is not provided".to_string());
            return Ok(json_error::<String>(error).into_response());
        }
    };

    match state.auth_service.verify_access_token(&token) {
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
