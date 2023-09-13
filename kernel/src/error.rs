#[derive(Debug, thiserror::Error)]
pub enum KernelError {
    #[error("Validation Error: {msg}")]
    Validation { msg: &'static str },
    #[error("Unsupported type conversion. from: {from} -> to: {to}")]
    UnSupportedTypeConversion {
        from: &'static str,
        to: &'static str
    },
    #[error(transparent)]
    Driver(anyhow::Error),
    #[error(transparent)]
    Internal(anyhow::Error)
}