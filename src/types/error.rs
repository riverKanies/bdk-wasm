use thiserror::Error;

#[derive(Error, Debug)]
pub enum WalletError {
    #[error("Failed to initialize storage: {0}")]
    Initialize(String),
    #[error("Failed to persist: {0}")]
    Persist(String),
    #[error("Failed to deserialize: {0}")]
    Deserialize(String),
    #[error("Failed to serialize: {0}")]
    Serialize(String),
    #[error("Promise returned error: {0}")]
    Future(String),
}
