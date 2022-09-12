use std::fmt;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum PngError {
    InvalidChunkType([u8; 4]),
}

impl fmt::Display for PngError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidChunkType(bytes) => {
                write!(f, "invalid chunk type {:02x?}", bytes)
            }
        }
    }
}

impl std::error::Error for PngError {}
