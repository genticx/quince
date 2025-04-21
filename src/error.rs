use thiserror::Error;

#[derive(Error, Debug)]
pub enum PinataError {
    #[error("Invalid API key or secret")]
    InvalidCredentials,

    #[error("Failed to pin file: {0}")]
    PinFileError(String),

    #[error("Failed to pin JSON: {0}")]
    PinJsonError(String),

    #[error("Failed to unpin: {0}")]
    UnpinError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Deserialization error: {0}")]
    DeserializationError(String),
}

pub type Result<T> = std::result::Result<T, PinataError>;
