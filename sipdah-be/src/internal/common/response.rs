use crate::internal::model::error::Error;
use crate::internal::model::web::ApiResponse;
use axum::http::StatusCode;
use axum::Json as AxumJson;
use serde::Serialize;

pub trait Json<T>
where
    T: Serialize,
{
    fn json_with(
        self,
        code: u16,
        message: String,
    ) -> (StatusCode, AxumJson<ApiResponse<Option<T>>>);

    fn json(self) -> (StatusCode, AxumJson<ApiResponse<Option<T>>>);
}

impl<T> Json<T> for Result<T, Error>
where
    T: Serialize,
{
    fn json_with(
        self,
        code: u16,
        message: String,
    ) -> (StatusCode, AxumJson<ApiResponse<Option<T>>>) {
        let result = match self {
            Ok(data) => json_success(code, data, message),
            Err(error) => json_error(error),
        };

        result
    }

    fn json(self) -> (StatusCode, AxumJson<ApiResponse<Option<T>>>) {
        self.json_with(200, "Success!".to_string())
    }
}

impl<T> Json<T> for Error
where
    T: Serialize,
{
    fn json_with(
        self,
        _code: u16,
        _message: String,
    ) -> (StatusCode, AxumJson<ApiResponse<Option<T>>>) {
        json_error::<T>(self)
    }

    fn json(self) -> (StatusCode, AxumJson<ApiResponse<Option<T>>>) {
        self.json_with(400, "".to_string())
    }
}

pub fn json_success<T>(
    status: u16,
    data: T,
    message: String,
) -> (StatusCode, AxumJson<ApiResponse<Option<T>>>)
where
    T: Serialize,
{
    (
        StatusCode::from_u16(status).unwrap(),
        AxumJson(ApiResponse {
            data: Some(data),
            status,
            message,
        }),
    )
}

pub fn json_error<T>(error: Error) -> (StatusCode, AxumJson<ApiResponse<Option<T>>>)
where
    T: Serialize,
{
    let (status, message) = match error {
        Error::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
        Error::Unauthorized(message) => (StatusCode::UNAUTHORIZED, message),
        Error::Forbidden(message) => (StatusCode::FORBIDDEN, message),
        Error::NotFound(message) => (StatusCode::NOT_FOUND, message),
        Error::Conflict(message) => (StatusCode::CONFLICT, message),
        Error::Internal(message) => (StatusCode::INTERNAL_SERVER_ERROR, message),
    };

    (
        status,
        AxumJson(ApiResponse {
            data: None,
            status: status.as_u16(),
            message,
        }),
    )
}
