mod csv_handler;
mod ratings;
mod cli;

use anyhow::{Result};
use clap::Parser;
use cli::{Cli, handle_command};

fn main() -> Result<()>{
    let cli = Cli::parse();
    handle_command(&cli.command)
}
