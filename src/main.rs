mod args;
mod chunk;
mod chunk_type;
mod commands;
mod error;
mod png;

pub use error::{Error, Result};

use args::{Cli, Commands};
use clap::Parser;
use commands::{decode, encode, print_chunks, remove};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encode(args) => encode(args),
        Commands::Decode(args) => decode(args),
        Commands::Remove(args) => remove(args),
        Commands::Print(args) => print_chunks(args),
    }
}
