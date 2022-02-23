use axum::{http::StatusCode, Json};
use serde_json::{json, Value};
use thiserror::Error;

pub use meter::MeterError;

mod meter;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Meter(#[from] MeterError),
    #[error(transparent)]
    AxumPath(#[from] axum::extract::rejection::PathRejection),
}

pub type Result<T> = std::result::Result<T, Error>;

pub type ApiError = (StatusCode, Json<Value>);
pub type ApiResult<T> = std::result::Result<T, ApiError>;

impl std::error::Error for MeterError {}

impl From<Error> for ApiError {
    fn from(err: Error) -> Self {
        // NOTE: Remove this eventually
        dbg!(&err);
        let status = match err {
            Error::Meter(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let payload = json!({"message": err.to_string()});
        (status, Json(payload))
    }
}
