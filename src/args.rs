#![allow(dead_code)]

use std::path::PathBuf;
use std::str::FromStr;

use clap::{Args, Parser, Subcommand};

use crate::chunk_type::ChunkType;

fn parse_chunk_type(s: &str) -> Result<ChunkType, String> {
    // Possible errors from `ChunkType::from_str`:
    // - TryFromSliceError (when `s` is not 4 bytes)
    // - PngMeError::InvalidChunkType (contains non-alphabetic bytes)
    // Since we use dyn Error here, we cannot match on error types.
    // TODO: wrap TryFromSliceError into PngMeError enum member
    ChunkType::from_str(s).map_err(|_| format!("must be 4 letters"))
}

#[derive(Parser, Debug)]
#[clap(about)]
pub struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Encode message into PNG file
    Encode(EncodeArgs),

    /// Decode message from PNG file
    Decode(DecodeArgs),

    /// Remove a chunk from PNG file
    Remove(RemoveArgs),

    /// Print PNG file as data stream
    Print(PrintArgs),
}

#[derive(Args, Debug)]
pub struct EncodeArgs {
    /// The PNG file to be encoded into
    file_path: PathBuf,

    /// The secret key (four alphabets)
    #[clap(value_parser = parse_chunk_type)]
    chunk_type: ChunkType,

    /// The secret message
    message: String,

    /// Optional output file path
    output_file: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct DecodeArgs {
    /// The PNG file to be decoded
    file_path: PathBuf,

    /// The secret key (four alphabets)
    #[clap(value_parser = parse_chunk_type)]
    chunk_type: ChunkType,
}

#[derive(Args, Debug)]
pub struct RemoveArgs {
    /// The PNG file to be operated on
    file_path: PathBuf,

    /// The secret key (four alphabets)
    #[clap(value_parser = parse_chunk_type)]
    chunk_type: ChunkType,
}

#[derive(Args, Debug)]
pub struct PrintArgs {
    /// The PNG file to be printed
    file_path: PathBuf,
}
