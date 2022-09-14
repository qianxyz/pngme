#![allow(dead_code)]

use std::path::PathBuf;
use std::str::FromStr;

use clap::{Parser, Subcommand};

use crate::chunk_type::ChunkType;

fn parse_chunk_type(s: &str) -> Result<ChunkType, String> {
    // Possible errors from `ChunkType::from_str`:
    // - TryFromSliceError (when `s` is not 4 bytes)
    // - PngMeError::InvalidChunkType (contains non-alphabetic bytes)
    // Since we use dyn Error here, we cannot match on error types.
    // TODO: wrap TryFromSliceError into PngMeError enum member
    ChunkType::from_str(s).map_err(|_| format!("invalid chunk type {}", s))
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
    Encode {
        /// The PNG file to be encoded into
        file_path: PathBuf,

        /// The secret key (four alphabets)
        #[clap(value_parser = parse_chunk_type)]
        chunk_type: ChunkType,

        /// The secret message
        message: String,

        /// Optional output file path
        output_file: Option<PathBuf>,
    },

    /// Decode message from PNG file
    Decode {
        /// The PNG file to be decoded
        file_path: PathBuf,

        /// The secret key (four alphabets)
        #[clap(value_parser = parse_chunk_type)]
        chunk_type: ChunkType,
    },

    /// Remove a chunk from PNG file
    Remove {
        /// The PNG file to be operated on
        file_path: PathBuf,

        /// The secret key (four alphabets)
        #[clap(value_parser = parse_chunk_type)]
        chunk_type: ChunkType,
    },

    /// Print PNG file as data stream
    Print {
        /// The PNG file to be printed
        file_path: PathBuf,
    },
}
