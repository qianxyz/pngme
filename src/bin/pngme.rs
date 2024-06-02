use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand};

use pngme::{decode, encode, print_chunks, remove};

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

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encode(args) => {
            let png_data = fs::read(&args.file_path)
                .with_context(|| format!("Failed to read file: {:?}", &args.file_path))?;

            let new_png_data = encode(png_data, Some(args.chunk_type), args.message)?;

            let output_file = match &args.output_file {
                Some(path) => path,
                None => &args.file_path,
            };

            fs::write(output_file, new_png_data)
                .with_context(|| format!("Failed to write file: {:?}", output_file))?;
        }

        Commands::Decode(args) => {
            let png_data = fs::read(&args.file_path)
                .with_context(|| format!("Failed to read file: {:?}", &args.file_path))?;

            let messages = decode(png_data, Some(args.chunk_type))?;

            for message in messages {
                println!("{}", message);
            }
        }
        Commands::Remove(args) => {
            let png_data = fs::read(&args.file_path)
                .with_context(|| format!("Failed to read file: {:?}", &args.file_path))?;

            let new_png_data = remove(png_data, Some(args.chunk_type))?;

            fs::write(&args.file_path, new_png_data)
                .with_context(|| format!("Failed to write file: {:?}", &args.file_path))?;
        }
        Commands::Print(args) => {
            let png_data = fs::read(&args.file_path)
                .with_context(|| format!("Failed to read file: {:?}", &args.file_path))?;

            print!("{}", print_chunks(png_data)?);
        }
    }

    Ok(())
}
