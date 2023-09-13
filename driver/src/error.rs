use kernel::error::KernelError;

#[derive(Debug, thiserror::Error)]
pub enum DriverError {
    #[error(transparent)]
    Sqlx(anyhow::Error),
    #[error(transparent)]
    Kernel(anyhow::Error)
}


impl From<KernelError> for DriverError {
    fn from(internal: KernelError) -> Self {
        Self::Kernel(anyhow::Error::new(internal))
    }
}

impl From<sqlx::Error> for DriverError {
    fn from(e: sqlx::Error) -> Self {
        Self::Sqlx(anyhow::Error::new(e))
    }
}

impl From<DriverError> for KernelError {
    fn from(value: DriverError) -> Self {
        match value {
            DriverError::Sqlx(e) => Self::Driver(e),
            DriverError::Kernel(e) => Self::Internal(e)
        }
    }
}