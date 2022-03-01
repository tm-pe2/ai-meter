use axum::{http::StatusCode, Json};
use serde_json::{json, Value};
use thiserror::Error;

// Rexport [`MeterError`] in crate::error
pub use meter::MeterError;

mod meter;

/// Error type of this create
#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Meter(#[from] MeterError),
    #[error(transparent)]
    AxumPath(#[from] axum::extract::rejection::PathRejection),
    #[error(transparent)]
    DieselResult(#[from] diesel::result::Error),
    #[error(transparent)]
    R2d2(#[from] r2d2::Error),
    #[error("invalid identifier")]
    InvalidIdentifier,
}

/// Type wrapper arround [`Error`]
pub type Result<T> = std::result::Result<T, Error>;

/// Return type of the api
pub type ApiError = (StatusCode, Json<Value>);

/// Type wrapper arround [`ApiError`]
pub type ApiResult<T> = std::result::Result<T, ApiError>;

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
