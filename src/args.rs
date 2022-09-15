use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(about)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
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
    pub file_path: PathBuf,

    /// The secret key (four alphabets)
    pub chunk_type: String,

    /// The secret message
    pub message: String,

    /// Optional output file path
    pub output_file: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct DecodeArgs {
    /// The PNG file to be decoded
    pub file_path: PathBuf,

    /// The secret key (four alphabets)
    pub chunk_type: String,
}

#[derive(Args, Debug)]
pub struct RemoveArgs {
    /// The PNG file to be operated on
    pub file_path: PathBuf,

    /// The secret key (four alphabets)
    pub chunk_type: String,
}

#[derive(Args, Debug)]
pub struct PrintArgs {
    /// The PNG file to be printed
    pub file_path: PathBuf,
}
