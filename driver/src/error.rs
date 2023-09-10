use kernel::error::KernelError;

#[derive(Debug, thiserror::Error)]
pub enum DriverError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error)
}

impl From<DriverError> for KernelError {
    fn from(value: DriverError) -> Self {
        match value {
            DriverError::Sqlx(e) => Self::Driver(anyhow::Error::new(e))
        }
    }
}