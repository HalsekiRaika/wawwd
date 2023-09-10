use kernel::error::KernelError;

#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error("From Kernel: {0}")]
    Kernel(anyhow::Error),
    #[error("cannot find `{entity}:{target}` in the following {method}.")]
    NotFound {
        entity: &'static str,
        method: &'static str,
        target: String
    },
    #[error(transparent)]
    Other(anyhow::Error)
}

impl From<KernelError> for ApplicationError {
    fn from(value: KernelError) -> Self {
        match value {
            KernelError::Validation { msg } => Self::Kernel(anyhow::Error::msg(msg)),
            KernelError::Driver(e) => Self::Other(e)
        }
    }
}