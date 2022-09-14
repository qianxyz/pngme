use std::fmt;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum PngMeError {
    InvalidChunkType([u8; 4]),
    InvalidCrc(u32, u32),
    ChunkTypeNotFound(String),
    InvalidHeader([u8; 8]),
}

impl fmt::Display for PngMeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidChunkType(bytes) => {
                write!(f, "invalid chunk type: {:02x?}", bytes)
            }
            Self::InvalidCrc(expected, found) => {
                write!(f, "invalid CRC: expect {}, found {}", expected, found)
            }
            Self::ChunkTypeNotFound(chunk_type) => {
                write!(f, "chunk type {} not found", chunk_type)
            }
            Self::InvalidHeader(header) => {
                write!(f, "invalid PNG header {:?}", header)
            }
        }
    }
}

impl std::error::Error for PngMeError {}
