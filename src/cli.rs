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
        rating: i8,
        properties: Vec<String>,
    },
    Show {
        category: String,

        #[arg(short, long)]
        sort: bool
    },
    Edit {
        category: String,
        name: String,
        score: i8
    }
}
