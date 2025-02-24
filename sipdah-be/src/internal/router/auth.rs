use crate::internal::common::response::{json_error, json_success};
use crate::internal::model::auth;
use crate::internal::model::auth::{SignInRequest, SignUpRequest};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use axum_extra::extract::CookieJar;
use std::sync::Arc;

#[derive(Clone)]
pub struct AuthState<T1>
where
    T1: auth::Service,
{
    pub auth_service: Arc<T1>,
}

pub async fn sign_up<T1: auth::Service>(
    jar: CookieJar,
    State(state): State<Arc<AuthState<T1>>>,
    Json(req): Json<SignUpRequest>,
) -> impl IntoResponse + Send {
    match state.auth_service.sign_up(&req).await {
        Ok(res) => {
            let cookie = axum_extra::extract::cookie::Cookie::build((
                "refresh_token",
                res.refresh_token.clone(),
            ))
            .path("/")
            .http_only(true)
            .build();
            let jar = jar.add(cookie);

            (jar, json_success(201, res, "Success!".to_string())).into_response()
        }
        Err(error) => json_error::<String>(error).into_response(),
    }
}

pub async fn sign_in<T1: auth::Service>(
    jar: CookieJar,
    State(state): State<Arc<AuthState<T1>>>,
    Json(req): Json<SignInRequest>,
) -> impl IntoResponse + Send {
    match state.auth_service.sign_in(&req).await {
        Ok(res) => {
            let cookie = axum_extra::extract::cookie::Cookie::build((
                "refresh_token",
                res.refresh_token.clone(),
            ))
            .path("/")
            .http_only(true)
            .build();
            let jar = jar.add(cookie);

            (jar, json_success(200, res, "Success!".to_string())).into_response()
        }
        Err(error) => json_error::<String>(error).into_response(),
    }
}
