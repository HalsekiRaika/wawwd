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
    Application(ApplicationError),
    #[error(transparent)]
    Driver(anyhow::Error),
    #[error(transparent)]
    Kernel(anyhow::Error),
    #[error("Required environment variable not set! `{0}` must set.")]
    EnvError(&'static str),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            ServerError::IO(e) => (StatusCode::BAD_REQUEST, e.to_string()),
            ServerError::HandlerInitialization(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            ServerError::Driver(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            ServerError::Application(e) => match e {
                ApplicationError::Kernel(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
                ApplicationError::NotFound{ target, .. } => (StatusCode::NOT_FOUND, target.to_string()),
                ApplicationError::Other(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            },
            ServerError::Kernel(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            ServerError::EnvError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };

        let json = json!({ "error": msg });

        (status, Json(json)).into_response()
    }
}

impl From<ApplicationError> for ServerError {
    fn from(value: ApplicationError) -> Self {
        Self::Application(value)
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
