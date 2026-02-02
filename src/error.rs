use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Command failed: {0}")]
    CommandFailed(String),
    #[error("Missing dependency: {0}")]
    MissingDependency(String),
    #[error("Network error: {0}")]
    Network(String),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
