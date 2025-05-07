use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name="ratings")]
#[command(about="A CLI tool to keep ratings for albums, movies, etc.", long_about=None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        category: String,
        name: String,
        icon: String,
        rating: i8,
        properties: Vec<String>,
    }
}
