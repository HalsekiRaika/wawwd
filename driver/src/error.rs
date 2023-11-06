use kernel::error::KernelError;

#[derive(Debug, thiserror::Error)]
pub enum DriverError {
    #[error("`{column}` is must not empty.")]
    Decoding { column: &'static str },
    #[error(transparent)]
    Sqlx(anyhow::Error),
    #[error(transparent)]
    S3(anyhow::Error),
    #[error(transparent)]
    Redis(anyhow::Error),
    #[error("Failed database initialization. {0}")]
    DataBaseInitialization(anyhow::Error),
    #[error(transparent)]
    Kernel(anyhow::Error),
}

impl From<KernelError> for DriverError {
    fn from(internal: KernelError) -> Self {
        Self::Kernel(anyhow::Error::new(internal))
    }
}

impl From<s3::error::S3Error> for DriverError {
    fn from(e: s3::error::S3Error) -> Self {
        Self::S3(anyhow::Error::new(e))
    }
}

impl From<sqlx::Error> for DriverError {
    fn from(e: sqlx::Error) -> Self {
        Self::Sqlx(anyhow::Error::new(e))
    }
}

impl From<deadpool_redis::PoolError> for DriverError {
    fn from(value: deadpool_redis::PoolError) -> Self {
        Self::Redis(anyhow::Error::new(value))
    }
}

impl From<deadpool_redis::redis::RedisError> for DriverError {
    fn from(value: deadpool_redis::redis::RedisError) -> Self {
        Self::Redis(anyhow::Error::new(value))
    }
}

impl From<DriverError> for KernelError {
    fn from(value: DriverError) -> Self {
        match value {
            DriverError::S3(e) => Self::Driver(e),
            DriverError::Sqlx(e) => Self::Driver(e),
            DriverError::Redis(e) => Self::Driver(e),
            DriverError::Kernel(e) => Self::Internal(e),
            DriverError::DataBaseInitialization(e) => Self::Internal(e),
            DriverError::Decoding { .. } => Self::Driver(anyhow::Error::new(value)),
        }
    }
}


#[derive(Debug)]
pub struct DriverErrorKind {
    pub kind: &'static str,
    pub error: DriverError,
}

impl DriverErrorKind {
    pub fn new(kind: &'static str, error: DriverError) -> Self {
        Self { kind, error }
    }
}

impl From<DriverError> for DriverErrorKind {
    fn from(value: DriverError) -> Self {
        match value {
            DriverError::Kernel(_) => Self::new("kernel", value),
            DriverError::Decoding { .. } => Self::new("decoding", value),
            _ => Self::new("database", value),
        }
    }
}