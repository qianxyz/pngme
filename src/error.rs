use thiserror::Error;
use wasm_bindgen::JsValue;

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid chunk type: {0:?}")]
    InvalidChunkTypeBytes([u8; 4]),
    #[error("invalid chunk type: {0}")]
    InvalidChunkTypeString(String),
    #[error("invalid CRC: expected {0:x}, got {1:x}")]
    InvalidCrc(u32, u32),
    #[error("invalid header: {0:?}")]
    InvalidHeader([u8; 8]),
    #[error("chunk type {0} not found")]
    ChunkTypeNotFound(String),
    #[error("chunk type {0} is reserved")]
    ReservedChunkType(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl Into<JsValue> for Error {
    fn into(self) -> JsValue {
        JsValue::from_str(&self.to_string())
    }
}
