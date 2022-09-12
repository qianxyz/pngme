pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum PngError {
    NonAlphabeticBytesError(u8),
}

impl std::fmt::Display for PngError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonAlphabeticBytesError(b) => {
                write!(f, "non-alphabetic byte {:#04x?} in chunk type", b)
            }
        }
    }
}

impl std::error::Error for PngError {}
