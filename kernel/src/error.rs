#[derive(Debug, thiserror::Error)]
pub enum KernelError {
    #[error("Validation Error: {msg}")]
    Validation { msg: &'static str },
    #[error(transparent)]
    Driver(anyhow::Error)
}