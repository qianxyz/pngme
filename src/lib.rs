mod chunk;
mod chunk_type;
mod error;
mod png;

use std::str::FromStr;

pub use chunk::Chunk;
pub use chunk_type::ChunkType;
pub use error::Error;
pub use png::Png;

use wasm_bindgen::prelude::*;

// https://www.w3.org/TR/2003/REC-PNG-20031110/#11Chunks
pub const RESERVED_CHUNK_TYPES: [&str; 18] = [
    "IHDR", "PLTE", "IDAT", "IEND", "tRNS", "cHRM", "gAMA", "iCCP", "sBIT", "sRGB", "tEXt", "zTXt",
    "iTXt", "bKGD", "hIST", "pHYs", "sPLT", "tIME",
];

/// The default chunk type when the user doesn't specify one.
/// This chunk type is ancillary (not critical), private, and safe to copy.
pub const DEFAULT_CHUNK_TYPE: &str = "ruSt";

/// Encodes a message into a PNG file and saves the result
#[wasm_bindgen]
pub fn encode(
    png_data: Vec<u8>,
    chunk_type: Option<String>,
    message: String,
) -> Result<Vec<u8>, Error> {
    let mut png = Png::try_from(png_data.as_ref())?;

    let chunk_type = match chunk_type {
        Some(chunk_type) => {
            if RESERVED_CHUNK_TYPES.contains(&chunk_type.as_str()) {
                return Err(Error::ReservedChunkType(chunk_type));
            }

            ChunkType::from_str(&chunk_type)?
        }
        None => ChunkType::from_str(DEFAULT_CHUNK_TYPE)?,
    };
    let chunk = Chunk::new(chunk_type, message.as_bytes().to_vec());

    png.append_chunk(chunk);

    Ok(png.as_bytes())
}

/// Searches for a message hidden in a PNG file and prints the message if found
#[wasm_bindgen]
pub fn decode(png_data: Vec<u8>, chunk_type: Option<String>) -> Result<Vec<String>, Error> {
    let png = Png::try_from(png_data.as_ref())?;

    let chunks = match chunk_type {
        Some(chunk_type) => png.chunks_by_type(&chunk_type),
        None => png.chunks_filtered(|chunk| {
            !RESERVED_CHUNK_TYPES.contains(&chunk.chunk_type().to_string().as_str())
        }),
    };

    Ok(chunks
        .iter()
        .map(|chunk| match chunk.data_as_string() {
            Ok(data) => data,
            Err(_) => format!("invalid utf-8 data: {:?}", chunk.data()),
        })
        .collect())
}

/// Removes a chunk from a PNG file and saves the result
#[wasm_bindgen]
pub fn remove(png_data: Vec<u8>, chunk_type: Option<String>) -> Result<Vec<u8>, Error> {
    let mut png = Png::try_from(png_data.as_ref())?;

    match chunk_type {
        Some(chunk_type) => {
            if RESERVED_CHUNK_TYPES.contains(&chunk_type.as_str()) {
                return Err(Error::ReservedChunkType(chunk_type));
            }

            png.remove_chunks(&chunk_type)
        }
        None => png.remove_chunks_filtered(|chunk| {
            !RESERVED_CHUNK_TYPES.contains(&chunk.chunk_type().to_string().as_str())
        }),
    };

    Ok(png.as_bytes())
}

/// Prints all of the chunks in a PNG file
#[wasm_bindgen]
pub fn print_chunks(png_data: Vec<u8>) -> Result<String, Error> {
    let png = Png::try_from(png_data.as_ref())?;

    Ok(png.to_string())
}
