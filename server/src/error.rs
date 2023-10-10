use application::error::ApplicationError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use driver::error::DriverError;
use kernel::error::KernelError;
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error(transparent)]
    IO(anyhow::Error),
    #[error(transparent)]
    HandlerInitialization(anyhow::Error),
    #[error(transparent)]
    Application(anyhow::Error),
    #[error(transparent)]
    Driver(anyhow::Error),
    #[error(transparent)]
    Kernel(anyhow::Error),
    #[error("Required environment variable not set! `{0}` must set.")]
    EnvError(&'static str),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let e = match self {
            ServerError::IO(e) => e.to_string(),
            ServerError::HandlerInitialization(e) => e.to_string(),
            ServerError::Driver(e) => e.to_string(),
            ServerError::Application(e) => e.to_string(),
            ServerError::Kernel(e) => e.to_string(),
            ServerError::EnvError(e) => e.to_string(),
        };

        let json = json!({ "error": e });

        (StatusCode::BAD_REQUEST, Json(json)).into_response()
    }
}

impl From<ApplicationError> for ServerError {
    fn from(value: ApplicationError) -> Self {
        Self::Application(anyhow::Error::new(value))
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
