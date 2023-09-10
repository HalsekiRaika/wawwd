#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error(transparent)]
    HandlerInitialization(anyhow::Error)
}