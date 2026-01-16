pub mod project_errors {
    use crate::models::brief_case::BriefCase;
    use crate::models::profile::Profile;
    use std::sync::{PoisonError, RwLockWriteGuard};
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

        #[error("Internal State Error: The {0} lock was poisoned")]
        LockPoisoned(String),
    }
}
