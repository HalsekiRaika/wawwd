use std::fmt::{Display, Formatter};
use kernel::error::{KernelError, KernelErrorKind};

#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error(transparent)]
    Kernel(KernelErrorKind),
    #[error("cannot find `{entity}:{target}` in the following {method}.")]
    NotFound {
        entity: &'static str,
        method: &'static str,
        target: String,
    },
    #[error(transparent)]
    Other(anyhow::Error),
}

impl From<KernelError> for ApplicationError {
    fn from(value: KernelError) -> Self {
        match value {
            KernelError::Driver(e) => Self::Other(e),
            _ => Self::Kernel(value.into()),
        }
    }
}


#[derive(Debug)]
pub struct ApplicationErrorKind {
    pub kind: String,
    pub error: ApplicationError,
}

impl ApplicationErrorKind {
    pub fn new(kind: impl Into<String>, error: ApplicationError) -> Self {
        Self { kind: kind.into(), error }
    }
}

impl Display for ApplicationErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.kind, self.error)
    }
}

impl std::error::Error for ApplicationErrorKind {}

impl From<ApplicationError> for ApplicationErrorKind {
    fn from(value: ApplicationError) -> Self {
        match value {
            ApplicationError::Kernel(_) => Self::new("kernel", value),
            ApplicationError::NotFound { entity, .. } => Self::new(format!("not_found_{}", entity), value),
            ApplicationError::Other(_) => Self::new("driver", value),
        }
    }
}