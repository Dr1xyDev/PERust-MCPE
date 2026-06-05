use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("Invalid packet ID: {0}")]
    InvalidPacketId(u8),

    #[error("Decode error: {0}")]
    DecodeError(String),

    #[error("Encode error: {0}")]
    EncodeError(String),

    #[error("Unsupported protocol version: {0}")]
    UnsupportedProtocol(i32),

    #[error("JWT error: {0}")]
    JwtError(String),

    #[error("Compression error")]
    CompressionError,

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Binary error: {0}")]
    BinaryError(#[from] perust_utils::binary::BinaryError),
}

impl From<ProtocolError> for std::io::Error {
    fn from(err: ProtocolError) -> Self {
        std::io::Error::new(std::io::ErrorKind::Other, err.to_string())
    }
}
