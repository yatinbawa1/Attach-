use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadError {
    #[error("File not found")]
    NotFound,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Deserialization error: {0}")]
    Deserialize(#[from] serde_json::Error),

    #[error("Unable to resolve config directory")]
    NoConfigDir,
}
#[derive(Debug, Error)]
pub enum WriteError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialize(#[from] serde_json::Error),

    #[error("Trying to write empty value to disk")]
    EmptyValue,
}
