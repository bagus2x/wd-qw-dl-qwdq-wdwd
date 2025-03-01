use crate::internal::common::response::{json_error, json_success};
use crate::internal::model::auth;
use crate::internal::model::auth::{RefreshTokenRequest, SignInRequest, SignUpRequest};
use crate::internal::model::error::Error;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use axum_extra::extract::cookie::SameSite;
use axum_extra::extract::{cookie, CookieJar};
use cookie::Cookie;
use std::sync::Arc;
use time::Duration;

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
            let refresh_token = Cookie::build(("refresh_token", res.refresh_token.clone()))
                .http_only(true)
                .same_site(SameSite::None)
                .max_age(Duration::days(7))
                .path("/")
                .build();
            let access_token = Cookie::build(("access_token", res.access_token.clone()))
                .http_only(true)
                .same_site(SameSite::None)
                .max_age(Duration::days(7))
                .path("/")
                .build();
            let is_signed_in = Cookie::build(("is_signed_in", "true"))
                .http_only(false)
                .same_site(SameSite::None)
                .max_age(Duration::days(7))
                .path("/")
                .build();

            let jar = jar.add(refresh_token).add(access_token).add(is_signed_in);

            (
                jar,
                json_success(200, res, "Signed up successfully!".to_string()),
            )
                .into_response()
        }
        Err(err) => json_error::<String>(err).into_response(),
    }
}

pub async fn sign_in<T1: auth::Service>(
    jar: CookieJar,
    State(state): State<Arc<AuthState<T1>>>,
    Json(req): Json<SignInRequest>,
) -> impl IntoResponse + Send {
    match state.auth_service.sign_in(&req).await {
        Ok(res) => {
            let refresh_token = Cookie::build(("refresh_token", res.refresh_token.clone()))
                .http_only(true)
                .same_site(SameSite::None)
                .max_age(Duration::days(7))
                .path("/")
                .build();
            let access_token = Cookie::build(("access_token", res.access_token.clone()))
                .http_only(true)
                .same_site(SameSite::None)
                .max_age(Duration::days(7))
                .path("/")
                .build();
            let is_signed_in = Cookie::build(("is_signed_in", "true"))
                .http_only(false)
                .same_site(SameSite::None)
                .max_age(Duration::days(7))
                .path("/")
                .build();

            let jar = jar.add(refresh_token).add(access_token).add(is_signed_in);

            (
                jar,
                json_success(200, res, "Signed up successfully!".to_string()),
            )
                .into_response()
        }
        Err(err) => json_error::<String>(err).into_response(),
    }
}

pub async fn refresh<T1: auth::Service>(
    jar: CookieJar,
    State(state): State<Arc<AuthState<T1>>>,
) -> impl IntoResponse + Send {
    let refresh_token = match jar.get("refresh_token").map(|c| c.value().to_string()) {
        Some(token) => token,
        None => {
            let error = Error::BadRequest("Cannot request without sign in".to_string());
            return json_error::<String>(error).into_response();
        }
    };

    match state
        .auth_service
        .refresh(&RefreshTokenRequest { refresh_token })
        .await
    {
        Ok(res) => {
            let refresh_token = Cookie::build(("refresh_token", res.refresh_token.clone()))
                .http_only(true)
                .same_site(SameSite::None)
                .max_age(Duration::days(7))
                .path("/")
                .build();
            let access_token = Cookie::build(("access_token", res.access_token.clone()))
                .http_only(true)
                .same_site(SameSite::None)
                .max_age(Duration::days(7))
                .path("/")
                .build();
            let is_signed_in = Cookie::build(("is_signed_in", "true"))
                .http_only(false)
                .same_site(SameSite::None)
                .max_age(Duration::days(7))
                .path("/")
                .build();

            let jar = jar.add(refresh_token).add(access_token).add(is_signed_in);

            (
                jar,
                json_success(200, res, "Signed up successfully!".to_string()),
            )
                .into_response()
        }
        Err(error) => {
            let refresh_token = Cookie::build(("refresh_token", ""))
                .http_only(true)
                .same_site(SameSite::None)
                .max_age(Duration::seconds(0))
                .path("/")
                .build();
            let access_token = Cookie::build(("access_token", ""))
                .http_only(true)
                .same_site(SameSite::None)
                .max_age(Duration::seconds(0))
                .path("/")
                .build();
            let is_signed_in = Cookie::build(("is_signed_in", ""))
                .http_only(true)
                .same_site(SameSite::None)
                .max_age(Duration::seconds(0))
                .path("/")
                .build();

            let jar = jar
                .remove(refresh_token)
                .add(access_token)
                .add(is_signed_in);

            (
                jar,
                json_success(200, (), "Signed out successfully".to_string()),
            )
                .into_response()
        },
    }
}

pub async fn sign_out<T1: auth::Service>(
    jar: CookieJar,
    State(state): State<Arc<AuthState<T1>>>,
) -> impl IntoResponse + Send {
    match state.auth_service.sign_out().await {
        Ok(_) => {
            let refresh_token = Cookie::build(("refresh_token", ""))
                .http_only(true)
                .same_site(SameSite::None)
                .max_age(Duration::seconds(0))
                .path("/")
                .build();
            let access_token = Cookie::build(("access_token", ""))
                .http_only(true)
                .same_site(SameSite::None)
                .max_age(Duration::seconds(0))
                .path("/")
                .build();
            let is_signed_in = Cookie::build(("is_signed_in", ""))
                .http_only(true)
                .same_site(SameSite::None)
                .max_age(Duration::seconds(0))
                .path("/")
                .build();

            let jar = jar
                .remove(refresh_token)
                .add(access_token)
                .add(is_signed_in);

            (
                jar,
                json_success(200, (), "Signed out successfully".to_string()),
            )
                .into_response()
        }
        Err(err) => json_error::<String>(err).into_response(),
    }
}
