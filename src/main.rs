mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use clap::Parser;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    args::Cli::parse();
    match args::Cli::parse().command {
        args::Commands::Encode(args) => commands::encode(args),
        args::Commands::Decode(args) => commands::decode(args),
        args::Commands::Remove(args) => commands::remove(args),
        args::Commands::Print(args) => commands::print_chunks(args),
    }
}
