#[derive(Debug, thiserror::Error)]
pub enum KernelError {
    #[error("Validation Error: {msg}")]
    Validation { msg: &'static str },
    #[error("Conflict in, `{entity}`. {msg}")]
    Conflict { entity: &'static str, msg: &'static str },
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
    #[error(transparent)]
    Driver(anyhow::Error),
    #[error(transparent)]
    Internal(anyhow::Error),
}
