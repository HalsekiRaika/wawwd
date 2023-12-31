use application::error::{ApplicationError, ApplicationErrorKind};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use driver::error::DriverError;
use kernel::error::{KernelError, KernelErrorKind};
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
    #[error("Not found, `{entity}`: target `{target}`")]
    NotFound {
        entity: &'static str,
        target: String,
    },
    #[error("Invalid Token: {0}")]
    UnAuthorize(KernelError),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let (status, error_name, msg) = match self {
            ServerError::IO(e) => (StatusCode::BAD_REQUEST, "invalid_data".to_string(), e.to_string()),
            ServerError::HandlerInitialization(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "server".to_string(), e.to_string())
            }
            ServerError::Driver(e) => (StatusCode::INTERNAL_SERVER_ERROR, "database".to_string(), e.to_string()),
            ServerError::Application(e) => {
                let ApplicationErrorKind { kind, error } = e.into();
                match error {
                    ApplicationError::Kernel(e) => {
                        let KernelErrorKind { kind, error } = e;
                        (StatusCode::BAD_REQUEST, kind, error.to_string())
                    },
                    ApplicationError::NotFound { .. } => (StatusCode::NOT_FOUND, kind, error.to_string()),
                    ApplicationError::Other(_) => (StatusCode::BAD_REQUEST, kind, error.to_string()),
                }
            },
            ServerError::Kernel(e) => (StatusCode::INTERNAL_SERVER_ERROR, "kernel".to_string(), e.to_string()),
            ServerError::EnvError(e) => (StatusCode::INTERNAL_SERVER_ERROR, "env".to_string(), e.to_string()),
            ServerError::NotFound { .. } => (StatusCode::NOT_FOUND, "not_found".to_string(), self.to_string()),
            ServerError::UnAuthorize(e) => (StatusCode::UNAUTHORIZED, "unauthorized".to_string(), e.to_string()),
        };

        let json = json!({ "error": error_name, "message": msg });

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
            DriverError::Decoding { .. } => ServerError::Driver(anyhow::Error::new(value)),
            DriverError::S3(e) => ServerError::Driver(e),
            DriverError::Redis(e) => ServerError::Driver(e),
        }
    }
}

impl From<KernelError> for ServerError {
    fn from(value: KernelError) -> Self {
        Self::Kernel(anyhow::Error::new(value))
    }
}

impl From<axum::headers::Error> for ServerError {
    fn from(value: axum::headers::Error) -> Self {
        Self::IO(anyhow::Error::new(value))
    }
}