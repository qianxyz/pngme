use std::fs;
use std::str::FromStr;

use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::error::PngMeError;
use crate::png::Png;
use crate::Result;

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: EncodeArgs) -> Result<()> {
    let mut png = Png::from_file(&args.file_path)?;

    let chunk_type = ChunkType::from_str(&args.chunk_type)?;
    let chunk = Chunk::new(chunk_type, args.message.into_bytes());

    png.append_chunk(chunk);

    let output_file = match &args.output_file {
        Some(path) => path,
        None => &args.file_path,
    };

    fs::write(output_file, png.as_bytes())?;

    Ok(())
}

/// Searches for a message hidden in a PNG file and prints the message if found
pub fn decode(args: DecodeArgs) -> Result<()> {
    let png = Png::from_file(&args.file_path)?;

    match png.chunk_by_type(&args.chunk_type) {
        Some(chunk) => {
            println!("{}", chunk.data_as_string()?);
            Ok(())
        }
        None => Err(PngMeError::ChunkTypeNotFound(args.chunk_type).into()),
    }
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: RemoveArgs) -> Result<()> {
    let mut png = Png::from_file(&args.file_path)?;

    png.remove_chunk(&args.chunk_type)?;

    fs::write(&args.file_path, png.as_bytes())?;

    Ok(())
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: PrintArgs) -> Result<()> {
    let png = Png::from_file(&args.file_path)?;

    println!("{}", png);

    Ok(())
}
