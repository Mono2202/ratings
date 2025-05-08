mod csv_handler;
mod ratings;
mod cli;

use anyhow::{Result, anyhow};
use clap::Parser;
use csv_handler::{edit_rating, read_ratings, write_ratings};
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
            write_ratings(format!("./db/{}.csv", category), &rating)?;
            println!("Rating added successfully!");
        },
        Commands::Show { category , sort} => {
            let mut ratings = read_ratings(format!("./db/{}.csv", category))?;
            if *sort {
                ratings.sort_by_key(|r| std::cmp::Reverse(r.rating))
            }
            display_ratings(&ratings);
        }
        Commands::Edit { category, name, score } => {
            let mut ratings = read_ratings(format!("./db/{}.csv", category))?;
            for rating in ratings.iter_mut() {
                if &rating.name == name {
                    rating.rating = *score;
                    edit_rating(format!("./db/{}.csv", category), &rating)?;
                    println!("Rating edited successfully!");
                    break
                }
            }
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
