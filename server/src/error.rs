use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use driver::error::DriverError;
use kernel::error::KernelError;

#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error(transparent)]
    HandlerInitialization(anyhow::Error),
    #[error(transparent)]
    Driver(anyhow::Error),
    #[error(transparent)]
    Kernel(anyhow::Error),
    #[error("Failed required environment variable not set! `{0}` should set.")]
    EnvError(&'static str),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let e = match self {
            ServerError::HandlerInitialization(e) => {
                e.to_string()
            }
            ServerError::Driver(e) => {
                e.to_string()
            }
            ServerError::Kernel(e) => {
                e.to_string()
            }
            ServerError::EnvError(e) => {
                e.to_string()
            }
        };

        let json = json!({ "error": e });

        (StatusCode::BAD_REQUEST, Json(json)).into_response()
    }
}

impl From<DriverError> for ServerError {
    fn from(value: DriverError) -> Self {
        match value {
            DriverError::DataBaseInitialization(e) => ServerError::HandlerInitialization(e),
            DriverError::Sqlx(e) => ServerError::Driver(e),
            DriverError::Kernel(e) => ServerError::Kernel(e),
        }
    }
}

impl From<KernelError> for ServerError {
    fn from(value: KernelError) -> Self {
        Self::Kernel(anyhow::Error::new(value))
    }
}