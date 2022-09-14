mod args;
mod chunk;
mod chunk_type;
mod commands;
mod error;
mod png;

pub use error::{Error, Result};

use clap::Parser;

fn main() -> Result<()> {
    let cli = args::Cli::parse();

    println!("{:?}", cli);

    Ok(())
}
