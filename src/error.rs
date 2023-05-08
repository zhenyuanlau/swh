use thiserror::Error;

#[derive(Error, Debug)]
pub enum SwhError {
    #[error("Environment not found. Reason: {0}")]
    EnvNotFound(String),
}
