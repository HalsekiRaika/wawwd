use std::fmt::{Display, Formatter};

#[derive(Debug, thiserror::Error)]
pub enum KernelError {
    #[error("Validation Error: {msg}")]
    Validation { msg: &'static str },
    #[error("Conflict in, `{entity}`. {msg}")]
    Conflict {
        entity: &'static str,
        msg: &'static str,
    },
    #[error("Try type conversion. from: `{from}` -> to: `{to}`, src: {source}")]
    TryConversion {
        from: &'static str,
        to: &'static str,
        source: anyhow::Error,
    },
    #[error("Unsupported type conversion. from: `{from}` -> to: `{to}`")]
    UnSupportedTypeConversion {
        from: &'static str,
        to: &'static str,
    },
    #[error("Invalid format. {msg}")]
    InvalidFormat {
        ty: &'static str,
        msg: anyhow::Error,
    },
    #[error(transparent)]
    Driver(anyhow::Error),
    #[error(transparent)]
    Internal(anyhow::Error),
}


#[derive(Debug)]
pub struct KernelErrorKind {
    pub kind: String,
    pub error: KernelError
}

impl KernelErrorKind {
    pub fn new(kind: impl Into<String>, error: KernelError) -> Self {
        Self { kind: kind.into(), error }
    }
}

impl Display for KernelErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.kind, self.error)
    }
}

impl std::error::Error for KernelErrorKind {}

impl From<KernelError> for KernelErrorKind {
    fn from(value: KernelError) -> Self {
        match value {
            KernelError::Validation { .. } => Self::new("validation", value),
            KernelError::Conflict { entity, .. } => Self::new(format!("conflict_{}", entity), value),
            KernelError::TryConversion { .. } => Self::new("try_conversion", value),
            KernelError::UnSupportedTypeConversion { .. } => Self::new("unsupported_type_conversion", value),
            KernelError::InvalidFormat { .. } => Self::new("invalid_format", value),
            KernelError::Driver(_) => Self::new("driver", value),
            KernelError::Internal(_) => Self::new("kernel", value),
        }
    }
}