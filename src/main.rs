mod csv_handler;
mod ratings;
mod cli;

use anyhow::{Result, anyhow};
use clap::Parser;
use csv_handler::{read_ratings, write_ratings};
use ratings::{display_ratings, Rating, RatingType};
use cli::{Cli, Commands};

fn main() -> Result<()>{
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { category, name , rating, properties} => {
            let rating_type = handle_category(category, properties)?;
            let rating = Rating {
                name: name.clone(),
                rating: *rating,
                rating_type: rating_type
            };
            println!("{}", rating);
            write_ratings(format!("./db/{}.csv", category), &rating)?;
        },
        Commands::Show { category } => {
            let ratings = read_ratings(format!("./db/{}.csv", category))?;
            display_ratings(&ratings);
        }
    }

    Ok(())
}

fn handle_category(category: &String, properties: &Vec<String>)  -> Result<RatingType> {
    match category.as_str() {
        "album" => Ok(RatingType::Album {
            artist: properties[0].clone()
        }),
        _ => Err(anyhow!("Invalid category: {}", category))
    }
}
