use std::string::FromUtf8Error;
use std::io;

/// Errors that can occur during NBT reading/writing operations.
#[derive(Debug, thiserror::Error)]
pub enum NbtError {
    #[error("Invalid tag type: {0}")]
    InvalidTagType(u8),

    #[error("Unexpected tag type")]
    UnexpectedTag,

    #[error("IO error: {0}")]
    IoError(#[from] io::Error),

    #[error("Binary error: {0}")]
    BinaryError(#[from] perust_utils::binary::BinaryError),

    #[error("UTF-8 error: {0}")]
    Utf8Error(#[from] FromUtf8Error),

    #[error("Unexpected end of data")]
    UnexpectedEof,

    #[error("Invalid compression")]
    InvalidCompression,

    #[error("{0}")]
    Custom(String),
}

impl From<&str> for NbtError {
    fn from(s: &str) -> Self {
        NbtError::Custom(s.to_string())
    }
}
